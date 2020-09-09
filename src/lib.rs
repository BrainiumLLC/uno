#[macro_use]
mod macros;

pub mod conv;

use self::conv::*;

unorm!(8);
unorm!(16);
unorm!(32);
unorm!(64);
unorm!(size);

impl From<Unorm8> for Unorm16 {
    fn from(n: Unorm8) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}

impl From<Unorm8> for Unorm32 {
    fn from(n: Unorm8) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}

impl From<Unorm16> for Unorm32 {
    fn from(n: Unorm16) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}

impl From<Unorm8> for Unorm64 {
    fn from(n: Unorm8) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}

impl From<Unorm16> for Unorm64 {
    fn from(n: Unorm16) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}

impl From<Unorm32> for Unorm64 {
    fn from(n: Unorm32) -> Self {
        Self::from_inner(conv::extend_unorm(n.to_inner()))
    }
}
