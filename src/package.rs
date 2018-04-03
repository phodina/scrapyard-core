use regex::Regex;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Package {
    LQFP(u16),
    TSSOP(u16),
    WLCSP(u16),
    UFQFPN(u16),
    TFBGA(u16),
    VFQFPN(u16),
    EWLCSP(u16),
    UFBGA(u16),
    LFBGA(u16),
    Unknown(u16),
}

impl Package {
    // TODO: Return error
    pub fn new(package: &str) -> Package {
        lazy_static! {
            static ref RE :Regex = Regex::new(r"([[:alpha:]]*)(\d*)").unwrap();
        }

        let caps = RE.captures(package).unwrap();

        let count = caps.get(2).unwrap().as_str().parse::<u16>().unwrap();

        match caps.get(1).unwrap().as_str() {
            "LQFP" => Package::LQFP(count),
            "TSSOP" => Package::TSSOP(count),
            "WLCSP" => Package::WLCSP(count),
            "UFQFPN" => Package::UFQFPN(count),
            "TFBGA" => Package::TFBGA(count),
            "VFQFPN" => Package::VFQFPN(count),
            "EWLCSP" => Package::EWLCSP(count),
            "UFBGA" => Package::UFBGA(count),
            "LFBGA" => Package::LFBGA(count),
            &_ => Package::Unknown(count),
        }
    }

    pub fn is_grid(&self) -> bool {
        match *self {
            Package::UFBGA(_) => true,
            Package::TFBGA(_) => true,
            Package::EWLCSP(_) => true,
            Package::WLCSP(_) => true,
            Package::LFBGA(_) => true,
            _ => false,
        }
    }

    pub fn pins(&self) -> u16 {
        match *self {
            Package::LQFP(count) => count,
            Package::TSSOP(count) => count,
            Package::WLCSP(count) => count,
            Package::UFQFPN(count) => count,
            Package::TFBGA(count) => count,
            Package::VFQFPN(count) => count,
            Package::EWLCSP(count) => count,
            Package::UFBGA(count) => count,
            Package::LFBGA(count) => count,
            Package::Unknown(count) => count,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn package_lqfp48() {
        let package_name = "LQFP48";

        let package = Package::new(package_name);

        assert_eq!(false, package.is_grid());
        assert_eq!(48, package.pins());
    }

    #[test]
    fn package_tfbga() {
        let package_name = "TFBGA144";

        let package = Package::new(package_name);

        assert_eq!(true, package.is_grid());
        assert_eq!(144, package.pins());
    }

    #[test]
    #[should_panic]
    fn package_fail() {
        let package_name = "";

        let package = Package::new(package_name);
    }
}
