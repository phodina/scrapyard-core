enum Package{}

#[derive(PartialEq, Debug)]
enum Position {
	Linear(u16),
	Grid(u8, u8)
}

struct IOPin {
	reset: bool,
	label: String
}

impl IOPin {

	fn reset(&mut self) {
		self.reset = true;
	}

	fn is_reset(&self) -> bool {
		self.reset
	}

	fn set_label (&mut self, label : &str) {

		self.label = label.to_string();
	}

	fn label(&self) -> &str {
	    &self.label
	}
}

enum Pin {
	NC {name : String, position : Position}, 
	IO {name : String, position : Position, params: Box<IOPin>},
	BOOT {name : String, position : Position}, 
	NRST {name : String, position : Position},
	POWER {name : String, position : Position}
}

impl Pin {

	fn name(&self) ->  &str {
		match *self {
			Pin::NC{ref name, ..} => &name,
			Pin::IO{ref name, ..} => &name,
			Pin::BOOT{ref name, ..} => &name,
			Pin::NRST{ref name, ..} => &name,
			Pin::POWER{ref name, ..} => &name,
		}
	}

	fn position(&self) -> &Position {
		match *self {
			Pin::NC{ref position, ..} => &position,
			Pin::IO{ref position, ..} => &position,
			Pin::BOOT{ref position, ..} => &position,
			Pin::NRST{ref position, ..} => &position,
			Pin::POWER{ref position, ..} => &position,
		}
	}

	fn reset(&mut self) {
		match *self {
		    Pin::IO{ref mut params, ..} => params.reset(),
		    _ => (),
		}
	}

	fn is_reset(&self) -> Option<bool> {
		match *self {
		    Pin::IO{ref params, ..} => Some(params.is_reset()),
		    _ => None,
		}
	}

	fn set_label(&mut self, label: &str) {
		match *self {
		    Pin::IO{ref mut params, ..} => params.set_label(label),
		    _ => (),
		}
	}

	fn label(&self) -> Option<&str> {
		match *self {
		    Pin::IO{ref params, ..} => Some(params.label()),
		    _ => None,
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn pin_ok() {

    	let pin = Pin::POWER{ name: "VDD".to_string(), position: Position::Grid(4,3)};

    	assert_eq!(pin.name(), "VDD");
        assert_eq!(*pin.position() , Position::Grid(4,3));
    }

	#[test]
    fn pin_reset() {

    	let pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3), params: Box::new(IOPin{reset : true, label : "".to_string()})};

    	assert_eq!(pin.name(), "PA3");
        assert_eq!(*pin.position() , Position::Grid(4,3));
    	assert_eq!(pin.is_reset(), Some(true));
    }    

	#[test]
    fn pin_label() {

    	let mut pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3), params: Box::new(IOPin{reset : true, label : "".to_string()})};

    	pin.set_label("PWM");

    	assert_eq!(pin.name(), "PA3");
        assert_eq!(*pin.position() , Position::Grid(4,3));
    	assert_eq!(pin.is_reset(), Some(true));
    	assert_eq!(pin.label(), Some("PWM"));
    }    
}
