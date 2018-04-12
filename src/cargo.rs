use mcu::{MCUConf, ARMCore, Core};

#[derive(Serialize, Deserialize, Debug)]
pub enum CrateType {
    Binary,
    Library,
}

#[derive(Serialize, Deserialize, Debug)]
struct SemVer {
    major: u8,
    minor: u8,
    patch: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    name: String,
    version: SemVer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo {
    name: String,
    crate_type: CrateType,
    version: SemVer,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    target: String,
}

impl Cargo {
    pub fn new(mcu_conf: &MCUConf) -> Cargo {
        Cargo {
            name: String::new(),
            crate_type: CrateType::Binary,
            version: SemVer {
                major: 0,
                minor: 1,
                patch: 0,
            },
            authors: Vec::new(),
            dependencies: Vec::new(),
            target: Cargo::set_target(mcu_conf.get_core()),
        }
    }

	// TODO: Handle floating point
    // thumbv7em-none-eabihf
    fn set_target(core: &Core) -> String {

    	match core {
            &Core::ARM(ref arm) => match arm {
                &ARMCore::CortexM0 => String::from("thumbv6m-none-eabi"),
                &ARMCore::CortexM3 => String::from("thumbv7m-none-eabi"),
                &ARMCore::CortexM4 => String::from("thumbv7em-none-eabi"),
                &ARMCore::CortexM7 => String::from("thumbv7em-none-eabi"),
            },
            &Core::AVR => String::new(),
            &Core::STM8 => String::new(),
            &Core::MSP430 => String::new(),
        }
    }

    pub fn get_crate_type(&self) -> &CrateType {
        &self.crate_type
    }
}