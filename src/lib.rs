//! There are no docs. Sorry...

#[macro_use]
mod macros;

pub mod conv;

use self::conv::*;

unorm!(8);
unorm!(16);
unorm!(32);
unorm!(64);
unorm!(size);

from!(Unorm8, Unorm16);

from!(Unorm8, Unorm32);
from!(Unorm16, Unorm32);

from!(Unorm8, Unorm64);
from!(Unorm16, Unorm64);
from!(Unorm32, Unorm64);

// usize is guaranteed to be 32 bits or more
from!(Unorm8, UnormSize);
from!(Unorm16, UnormSize);
