use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::{FixedPoint, FixedPointValue};

// See:
// https://docs.rs/rand/latest/rand/distributions/uniform/index.html#extending-uniform-to-support-a-custom-type

#[derive(Clone, Copy, Debug)]
pub struct UniformFixedPoint<T: FixedPointValue> {
    low: FixedPoint<T>,
    high: FixedPoint<T>,
}

impl<T: FixedPointValue> SampleUniform for FixedPoint<T> {
    type Sampler = UniformFixedPoint<T>;
}

impl<T: FixedPointValue> UniformSampler for UniformFixedPoint<T> {
    type X = FixedPoint<T>;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow() - FixedPoint::new(1);
        if low >= high {
            panic!(
                r#"UniformFixedPoint::new_inclusive called with invalid range:
    low: {low:?}
    high: {high:?}"#
            );
        }
        UniformFixedPoint { low, high }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if low >= high {
            panic!(
                r#"UniformFixedPoint::new_inclusive called with invalid range:
    low: {low:?}
    high: {high:?}"#
            );
        }
        UniformFixedPoint { low, high }
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FixedPoint<T> {
        let size = self.high.abs_diff(self.low);
        if size.is_zero() {
            panic!("UniformFixedPoint::sample called with size zero.");
        }
        let value = FixedPoint::new(rng.gen::<[u8; 32]>());
        let narrowed = value % size;
        let max = FixedPoint::<T>::MAX;
        let raw = if narrowed <= max.unsigned_abs() {
            self.low.raw() + T::from_u256(narrowed.raw()).unwrap()
        } else {
            let abs_low = self.low.unsigned_abs();
            let abs_diff = narrowed.abs_diff(abs_low);
            T::from_u256(abs_diff.raw()).unwrap()
        };
        FixedPoint::new(raw)
    }
}

impl<T: FixedPointValue> Distribution<FixedPoint<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FixedPoint<T> {
        rng.gen_range(FixedPoint::<T>::MIN..=FixedPoint::<T>::MAX)
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::{I256, U256};
    use eyre::Result;
    use rand::thread_rng;

    use super::*;
    use crate::{fixed, fixed_i128, fixed_u128, FixedPoint};

    #[test]
    fn test_invalid_range_failure() {
        let low = fixed_i128!(1);
        let high = fixed_i128!(1);

        // new
        assert!(
            std::panic::catch_unwind(|| { UniformFixedPoint::new(low, high + 1.into()) }).is_err()
        );

        // new_inclusive
        assert!(
            std::panic::catch_unwind(|| { UniformFixedPoint::new_inclusive(low, high) }).is_err()
        );

        // sample
        assert!(std::panic::catch_unwind(|| {
            let mut rng = thread_rng();
            let sample = UniformFixedPoint { low, high };
            sample.sample(&mut rng);
        })
        .is_err());
    }

    #[test]
    fn fuzz_gen() -> Result<()> {
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let uint: FixedPoint<U256> = rng.gen();
            assert!(uint >= FixedPoint::<U256>::MIN);
            assert!(uint <= FixedPoint::<U256>::MAX);

            let int: FixedPoint<I256> = rng.gen();
            assert!(int >= FixedPoint::<I256>::MIN);
            assert!(int <= FixedPoint::<I256>::MAX);
        }
        Ok(())
    }

    #[test]
    fn fuzz_gen_range() -> Result<()> {
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            // simple uint
            let low = fixed_u128!(0);
            let high = fixed!(1_000);
            let uint = rng.gen_range(low..=high);
            assert!(uint >= low);
            assert!(uint <= high);

            // simple int
            let low = fixed_i128!(-1_000);
            let high = fixed!(1_000);
            let int = rng.gen_range(low..=high);
            assert!(int >= low);
            assert!(int <= high);

            // < 0 int
            let low = fixed_i128!(-1_000);
            let high = fixed!(-100);
            let int = rng.gen_range(low..=high);
            assert!(int >= low);
            assert!(int <= high);

            // <= 0 int
            let low = FixedPoint::<i128>::MIN;
            let high = fixed!(0);
            let int = rng.gen_range(low..=high);
            assert!(int >= low);
            assert!(int <= high);

            // > 0 int
            let low = fixed_i128!(100);
            let high = fixed!(1_000);
            let int = rng.gen_range(low..=high);
            assert!(int >= low);
            assert!(int <= high);
        }
        Ok(())
    }
}
