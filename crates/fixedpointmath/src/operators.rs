use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::{FixedPoint, FixedPointValue};

impl<T: FixedPointValue> Neg for FixedPoint<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(self.raw().flip_sign()).unwrap()
    }
}

/// Takes a mapping of operator traits to `FixedPoint` methods and implements
/// the operator and assignment operator for each one.
macro_rules! impl_mapped_operator {
    ($($trait:ident => $fn:ident),*) => {
        $(
            paste::paste! {

                impl<T: FixedPointValue> std::ops::$trait for FixedPoint<T> {
                    type Output = Self;

                    fn [<$trait:lower>](self, other: Self) -> Self::Output {
                        self.$fn(other).unwrap()
                    }
                }

                impl<T: FixedPointValue> std::ops::[<$trait Assign>] for FixedPoint<T> {
                    fn [<$trait:lower _assign>](&mut self, other: Self) {
                        *self = self.[<$trait:lower>](other);
                    }
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

impl_mapped_operator!(
    // use `mul_down` for `*` and `*=`.
    Mul => mul_down,
    // use `div_down` for `/` and `/=`.
    Div => div_down
);

/// Takes a list of operator traits and implements the operator and assignment
/// operator for each one by forwarding to the corresponding method on the
/// underlying `FixedPointValue`.
macro_rules! impl_forwarded_operator {
    ($($trait:ident),*) => {
        $(
            paste::paste! {

                impl<T: FixedPointValue> std::ops::$trait for FixedPoint<T> {
                    type Output = Self;

                    fn [<$trait:lower>](self, other: Self) -> Self::Output {
                        Self::new(self.raw().[<$trait:lower>](other.raw())).unwrap()
                    }
                }

                impl<T: FixedPointValue> std::ops::[<$trait Assign>] for FixedPoint<T> {
                    fn [<$trait:lower _assign>](&mut self, other: Self) {
                        *self = self.[<$trait:lower>](other);
                    }
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

// Forward these operators to the underlying `FixedPointValue`.
impl_forwarded_operator!(Add, Sub, Rem);
