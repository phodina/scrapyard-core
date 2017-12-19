enum Package{}

#[derive(PartialEq, Debug)]
enum Position {
	Linear(u16),
	Grid(u8, u8)
}

struct IOPin {
	reset: bool,
	label: String,
	signals: Vec<String>,
	current: Option<usize>
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

	fn signals(&self) -> &Vec<String> {
		&self.signals
	}

	fn select_signal(&mut self, signal: &str) -> bool {
		
		let idx = self.signals.iter().position(|ref r| r == &signal);
		match idx {
			Some(signal) => { 	self.current = Some(signal);
								true },
			None => false,
		}
	}

	fn current_signal(&self) -> Option<&str> {
	
		match self.current {
			Some(idx) => Some(&self.signals[idx]),
			None => None
		}
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

	pub fn name(&self) ->  &str {
		match *self {
			Pin::NC{ref name, ..} => &name,
			Pin::IO{ref name, ..} => &name,
			Pin::BOOT{ref name, ..} => &name,
			Pin::NRST{ref name, ..} => &name,
			Pin::POWER{ref name, ..} => &name,
		}
	}

	pub fn position(&self) -> &Position {
		match *self {
			Pin::NC{ref position, ..} => &position,
			Pin::IO{ref position, ..} => &position,
			Pin::BOOT{ref position, ..} => &position,
			Pin::NRST{ref position, ..} => &position,
			Pin::POWER{ref position, ..} => &position,
		}
	}

	pub fn reset(&mut self) {
		match *self {
		    Pin::IO{ref mut params, ..} => params.reset(),
		    _ => (),
		}
	}

	pub fn is_reset(&self) -> Option<bool> {
		match *self {
		    Pin::IO{ref params, ..} => Some(params.is_reset()),
		    _ => None,
		}
	}

	pub fn set_label(&mut self, label: &str) {
		match *self {
		    Pin::IO{ref mut params, ..} => params.set_label(label),
		    _ => (),
		}
	}

	pub fn label(&self) -> Option<&str> {
		match *self {
		    Pin::IO{ref params, ..} => Some(params.label()),
		    _ => None,
		}
	}

	pub fn signals(&self) -> Option<&Vec<String>>{
		match *self {
		    Pin::IO{ref params, ..} => Some(params.signals()),
		    _ => None,
		}
	}

	pub fn select_signal(& mut self, signal: &str) -> Option<bool> {
		match *self {
		    Pin::IO{ref mut params, ..} => Some(params.select_signal(signal)),
		    _ => None,
		}
	}

	pub fn current_signal(&self) -> Option<&str> {
		match *self {
		    Pin::IO{ref params, ..} => params.current_signal(),
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

    	let pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3), params: Box::new(IOPin{reset : true, label : "".to_string(), signals: vec![], current: None})};

    	assert_eq!(pin.is_reset(), Some(true));
    }    

	#[test]
    fn pin_label() {

    	let mut pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3), params: Box::new(IOPin{reset : true, label : "".to_string(), signals: vec![], current: None})};

    	pin.set_label("PWM");

    	assert_eq!(pin.label(), Some("PWM"));
    }  

	#[test]
    fn pin_signals() {

    	let mut pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3),
    					params: Box::new(IOPin{	reset : true,
    											label : "".to_string(),
    											signals: vec!["Input".to_string(), "Output".to_string(), "EXTI".to_string()],
    											current: Some(0)})};

    	let signals = pin.signals().unwrap();

    	assert_eq!(*signals, vec!["Input".to_string(), "Output".to_string(), "EXTI".to_string()]);
    }   

    #[test]
    fn pin_select_signal_present() {

    	let mut pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3),
    					params: Box::new(IOPin{	reset : true,
    											label : "".to_string(),
    											signals: vec!["Input".to_string(), "Output".to_string(), "EXTI".to_string()],
    											current: Some(0)})};

    	let ret = pin.select_signal("Output");

    	assert_eq!(ret, Some(true));
    	assert_eq!(pin.current_signal(), Some("Output"));
    }

    #[test]
    fn pin_select_signal_missing() {

    	let mut pin = Pin::IO{ name: "PA3".to_string(), position: Position::Grid(4,3),
    					params: Box::new(IOPin{	reset : true,
    											label : "".to_string(),
    											signals: vec!["Input".to_string(), "Output".to_string(), "EXTI".to_string()],
    											current: Some(0)})};

    	let ret = pin.select_signal("Missing");

		assert_eq!(ret, Some(false));
    	assert_eq!(pin.current_signal(), Some("Input"));
    }   
}
