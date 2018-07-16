use mcu::{ARMCore, Core, MCUConf};

#[derive(Serialize, Deserialize, Debug)]
pub enum CrateType {
    Binary,
    Library,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemVer {
    major: u8,
    minor: u8,
    patch: u8,
}

impl SemVer {
    pub fn new(major: u8, minor: u8, patch: u8) -> SemVer {
        SemVer {
            major: major,
            minor: minor,
            patch: patch,
        }
    }

    pub fn get_major(&self) -> u8 {
        self.major
    }

    pub fn get_minor(&self) -> u8 {
        self.minor
    }

    pub fn get_patch(&self) -> u8 {
        self.patch
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    name: String,
    version: SemVer,
}

impl Dependency {
    pub fn new(name: &str, version: SemVer) -> Dependency {
        Dependency {
            name: name.to_owned(),
            version: version,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &SemVer {
        &self.version
    }
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

    pub fn set_crate_name(&mut self, name: &str) {
        self.name = name.to_owned()
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

    pub fn add_crate(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    pub fn rm_crate(&mut self) {
        unimplemented!();
    }
}
