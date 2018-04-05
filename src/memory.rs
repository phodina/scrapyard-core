#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Memory {
    Flash { start: u32, size: u32 },
    Eeprom { start: u32, size: u32 },
    Ram { start: u32, size: u32 },
}

impl Memory {
    pub fn start(&self) -> u32 {
        match *self {
            Memory::Flash { start, .. } => start,
            Memory::Eeprom { start, .. } => start,
            Memory::Ram { start, .. } => start,
        }
    }

    pub fn end(&self) -> u32 {
        match *self {
            Memory::Flash { start, size } => start + size,
            Memory::Eeprom { start, size } => start + size,
            Memory::Ram { start, size } => start + size,
        }
    }

    pub fn size(&self) -> u32 {
        match *self {
            Memory::Flash { size, .. } => size,
            Memory::Eeprom { size, .. } => size,
            Memory::Ram { size, .. } => size,
        }
    }
}
