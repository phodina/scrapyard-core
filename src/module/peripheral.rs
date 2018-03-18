use std::fs::File;

pub struct Peripheral {
    name: String,
    config: String,
    enabled: bool,
    configured: bool,
}

impl Peripheral {
    pub fn new(name: &str, config: &str) -> Peripheral {
        Peripheral {
            name: String::from(name),
            config: String::from(config),
            enabled: false,
            configured: false,
        }
    }

    // TODO: Handle error
    pub fn setup(&self) {
        let file = File::open(&self.config);
    }
}
