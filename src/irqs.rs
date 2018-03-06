use std::error::Error;
use std::fs::File;
use std::path::Path;

use serde_json;

struct Interrupt {}

struct Interrupts {
    irqs: Vec<Interrupt>,
}

#[derive(Deserialize, Debug)]
pub struct IRQ {
    Desc: String,
    Value: String,
}

#[derive(Deserialize, Debug)]
pub struct IRQS {
    #[serde(rename = "IRQS")] pub irqs: Vec<IRQ>,
}

struct InterruptBuilder {
    irqs: IRQS,
}

impl InterruptBuilder {
    pub fn new(path: &Path) -> Result<InterruptBuilder, Box<Error>> {
        let file = File::open(path)?;
        let irqs: IRQS = serde_json::from_reader(file)?;

        Ok(InterruptBuilder { irqs: irqs })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn load_irqs() {
        let sample = Path::new("./samples/NVIC-STM32F042_Modes.json");
        let irqs = InterruptBuilder::new(sample);

        assert!(irqs.is_ok());
        }
    }
