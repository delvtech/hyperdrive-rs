use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use async_trait::async_trait;
use ethers::{
    providers::{Middleware, MiddlewareError, PendingTransaction},
    types::{transaction::eip2718::TypedTransaction, *},
};
use thiserror::Error;

/// The default number of times the nonce manager will retry sending a
/// transaction after a "nonce too low" or "nonce too high" error.
const NONCE_MANAGER_RETRIES: usize = 5;

#[derive(Debug)]
/// Middleware used for calculating nonces locally, useful for signing multiple
/// consecutive transactions without waiting for them to hit the mempool
pub struct NonceManagerMiddleware<M> {
    inner: M,
    init_guard: futures_locks::Mutex<()>,
    initialized: AtomicBool,
    nonce: AtomicU64,
    address: Address,
    retries: usize,
}

impl<M> NonceManagerMiddleware<M>
where
    M: Middleware,
{
    /// Instantiates the nonce manager with a 0 nonce. The `address` should be the
    /// address which you'll be sending transactions from
    pub fn new(inner: M, address: Address, maybe_retries: Option<usize>) -> Self {
        Self {
            inner,
            init_guard: Default::default(),
            initialized: Default::default(),
            nonce: Default::default(),
            address,
            retries: maybe_retries.unwrap_or(NONCE_MANAGER_RETRIES),
        }
    }

    /// Gets the address.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the next nonce to be used
    pub fn next(&self) -> U256 {
        let nonce = self.nonce.fetch_add(1, Ordering::SeqCst);
        nonce.into()
    }

    /// Retrieves the transaction count for initialization on first call to
    /// manager. Returns the current nonce.
    pub async fn initialize_nonce(
        &self,
        block: Option<BlockId>,
    ) -> Result<U256, NonceManagerError<M>> {
        if self.initialized.load(Ordering::SeqCst) {
            // return current nonce
            return Ok(self.nonce.load(Ordering::SeqCst).into());
        }

        let _guard = self.init_guard.lock().await;

        // do this again in case multiple tasks enter this codepath
        if self.initialized.load(Ordering::SeqCst) {
            // return current nonce
            return Ok(self.nonce.load(Ordering::SeqCst).into());
        }

        // initialize the nonce the first time the manager is called
        let nonce = self
            .inner
            .get_transaction_count(self.address, block)
            .await
            .map_err(MiddlewareError::from_err)?;
        self.nonce.store(nonce.as_u64(), Ordering::SeqCst);
        self.initialized.store(true, Ordering::SeqCst);
        Ok(nonce)
    } // guard dropped here

    /// Resets the nonce to the current transaction count. This can be used to
    /// reset the nonce manager when the EVM snapshot is reverted.
    pub async fn reset_nonce(&self, block: Option<BlockId>) -> Result<(), NonceManagerError<M>> {
        // Fail if we're attempting to reset before initialization.
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(NonceManagerError::ResetError);
        }

        // Reset the nonce to the current transaction count.
        let nonce = self
            .inner
            .get_transaction_count(self.address, block)
            .await
            .map_err(MiddlewareError::from_err)?;
        self.nonce.store(nonce.as_u64(), Ordering::SeqCst);

        Ok(())
    }

    async fn get_transaction_count_with_manager(
        &self,
        block: Option<BlockId>,
    ) -> Result<U256, NonceManagerError<M>> {
        // initialize the nonce the first time the manager is called
        if !self.initialized.load(Ordering::SeqCst) {
            let nonce = self
                .inner
                .get_transaction_count(self.address, block)
                .await
                .map_err(MiddlewareError::from_err)?;
            self.nonce.store(nonce.as_u64(), Ordering::SeqCst);
            self.initialized.store(true, Ordering::SeqCst);
        }

        Ok(self.next())
    }
}

#[derive(Error, Debug)]
/// Thrown when an error happens at the Nonce Manager
pub enum NonceManagerError<M: Middleware> {
    /// Thrown when the nonce manager fails to be reset. This occurs when reset
    /// is called before initialization.
    #[error("ResetError")]
    ResetError,
    /// Thrown when the nonce manager exceeds the maximum number of retries
    /// while submitting transactions.
    #[error("RetryError")]
    RetryError,
    /// Thrown when the internal middleware errors
    #[error("{0}")]
    MiddlewareError(M::Error),
}

impl<M: Middleware> MiddlewareError for NonceManagerError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> Self {
        NonceManagerError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            NonceManagerError::ResetError => None,
            NonceManagerError::RetryError => None,
            NonceManagerError::MiddlewareError(e) => Some(e),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<M> Middleware for NonceManagerMiddleware<M>
where
    M: Middleware,
{
    type Error = NonceManagerError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }

    async fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        if tx.nonce().is_none() {
            tx.set_nonce(self.get_transaction_count_with_manager(block).await?);
        }

        Ok(self
            .inner()
            .fill_transaction(tx, block)
            .await
            .map_err(MiddlewareError::from_err)?)
    }

    /// Signs and broadcasts the transaction. The optional parameter `block` can be passed so that
    /// gas cost and nonce calculations take it into account. For simple transactions this can be
    /// left to `None`.
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        let mut tx = tx.into();

        if tx.nonce().is_none() {
            tx.set_nonce(self.get_transaction_count_with_manager(block).await?);
        }

        // Attempt to submit the transaction. If there are nonce management
        // errors, these will be handled up to the maximum number of retries.
        for _ in 0..self.retries {
            // Send the transaction and handle the result.
            match self.inner.send_transaction(tx.clone(), block).await {
                Ok(tx_hash) => return Ok(tx_hash),
                Err(err) => {
                    // If the error isn't a nonce too low error or a nonce too
                    // high error, we can't proceed.
                    let err_str = err.to_string().to_lowercase();
                    if !err_str.contains("nonce too low") && !err_str.contains("nonce too high") {
                        return Err(MiddlewareError::from_err(err));
                    }
                }
            }

            // Get the current transaction count. If the current transaction
            // count equals the value stored in the nonce manager, we update the
            // nonce manager to avoid being stuck. Otherwise, we update the
            // nonce manager with the current transaction count.
            let mut nonce = self
                .get_transaction_count(self.address, block)
                .await?
                .as_u64();
            if nonce == self.nonce.load(Ordering::SeqCst) {
                nonce += 1;
            }
            self.nonce.store(nonce, Ordering::SeqCst);
            tx.set_nonce(nonce);
        }

        // Return a retry error since we exceeded the maximum amount of retries.
        Err(NonceManagerError::RetryError)
    }
}
