use std::fmt;

use ethers::types::Sign;

/// The sign associated with a fixed point number.
// NOTE: Order is important for the `PartialOrd` and `Ord` traits.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FixedPointSign {
    Negative,
    Positive,
}

impl FixedPointSign {
    /// Returns the sign's opposite.
    pub fn flip(self) -> FixedPointSign {
        match self {
            FixedPointSign::Negative => FixedPointSign::Positive,
            FixedPointSign::Positive => FixedPointSign::Negative,
        }
    }

    /// Returns the sign's opposite if `should_flip` is true.
    pub fn flip_if(self, should_flip: bool) -> FixedPointSign {
        if should_flip {
            self.flip()
        } else {
            self
        }
    }

    pub fn is_negative(self) -> bool {
        self == FixedPointSign::Negative
    }

    pub fn is_positive(self) -> bool {
        !self.is_negative()
    }
}

// Formatting //

impl fmt::Display for FixedPointSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixedPointSign::Negative => write!(f, "-"),
            FixedPointSign::Positive => write!(f, ""),
        }
    }
}

impl fmt::Debug for FixedPointSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixedPointSign::Negative => write!(f, "Negative"),
            FixedPointSign::Positive => write!(f, "Positive"),
        }
    }
}

// Conversions //

impl From<bool> for FixedPointSign {
    fn from(b: bool) -> FixedPointSign {
        match b {
            false => FixedPointSign::Negative,
            true => FixedPointSign::Positive,
        }
    }
}

impl From<i8> for FixedPointSign {
    fn from(n: i8) -> FixedPointSign {
        (n >= 0).into()
    }
}

impl From<Sign> for FixedPointSign {
    fn from(sign: Sign) -> FixedPointSign {
        match sign {
            Sign::Negative => FixedPointSign::Negative,
            Sign::Positive => FixedPointSign::Positive,
        }
    }
}

impl From<FixedPointSign> for bool {
    fn from(sign: FixedPointSign) -> bool {
        match sign {
            FixedPointSign::Negative => false,
            FixedPointSign::Positive => true,
        }
    }
}

impl From<FixedPointSign> for Sign {
    fn from(sign: FixedPointSign) -> Sign {
        match sign {
            FixedPointSign::Negative => Sign::Negative,
            FixedPointSign::Positive => Sign::Positive,
        }
    }
}
