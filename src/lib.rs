#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

pub mod mcu;
pub mod package;
pub mod module;

pub mod irqs;

pub mod pins;
pub mod pin;

pub use mcu::MCUBuilder;
