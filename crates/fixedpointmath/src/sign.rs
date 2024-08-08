use std::fmt;

use ethers::types::Sign;

/// The sign associated with a fixed point number.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FixedPointSign {
    Positive,
    Negative,
}

impl FixedPointSign {
    /// Returns the sign's opposite.
    pub fn flip(self) -> FixedPointSign {
        match self {
            FixedPointSign::Positive => FixedPointSign::Negative,
            FixedPointSign::Negative => FixedPointSign::Positive,
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
}

// Formatting //

impl fmt::Display for FixedPointSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixedPointSign::Positive => write!(f, ""),
            FixedPointSign::Negative => write!(f, "-"),
        }
    }
}

impl fmt::Debug for FixedPointSign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FixedPointSign::Positive => write!(f, "Positive"),
            FixedPointSign::Negative => write!(f, "Negative"),
        }
    }
}

// Conversions //

impl From<bool> for FixedPointSign {
    fn from(b: bool) -> FixedPointSign {
        match b {
            true => FixedPointSign::Positive,
            false => FixedPointSign::Negative,
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
            Sign::Positive => FixedPointSign::Positive,
            Sign::Negative => FixedPointSign::Negative,
        }
    }
}

impl From<FixedPointSign> for bool {
    fn from(sign: FixedPointSign) -> bool {
        match sign {
            FixedPointSign::Positive => true,
            FixedPointSign::Negative => false,
        }
    }
}

impl From<FixedPointSign> for Sign {
    fn from(sign: FixedPointSign) -> Sign {
        match sign {
            FixedPointSign::Positive => Sign::Positive,
            FixedPointSign::Negative => Sign::Negative,
        }
    }
}
