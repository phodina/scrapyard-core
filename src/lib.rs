#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(feature = "cc")]
pub mod interface;
pub mod mcu;
pub mod memory;
pub mod package;
pub mod module;
mod errors;


pub mod irqs;

pub mod pins;
pub mod pin;

use errors::*;

pub use mcu::MCU;
pub use mcu::MCUConf;

#[cfg(feature = "cc")]
pub use interface::*;
