use std::fs::File;

pub struct Peripheral {
    name: String,
    config: String,
}

impl Peripheral {
    pub fn new(name: &str, config: &str) -> Peripheral {
        Peripheral {
            name: String::from(name),
            config: String::from(config),
        }
    }

    // TODO: Handle error
    pub fn setup(&self) {
        let file = File::open(&self.config);
    }
}
