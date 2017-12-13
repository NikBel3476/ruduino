//! Modules that can be implemented for specific cores.

pub use self::spi::HardwareSpi;
pub use self::timer::{Timer8, Timer8Setup, Timer16, Timer16Setup};

mod spi;
mod timer;

