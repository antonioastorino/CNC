use super::cnc_error::*;
use super::cnc_geometry::*;

use std::io::Write;

pub struct CncEngine {
	curr_location: (f32, f32, f32),
	initialized: bool,
	port: std::boxed::Box<dyn serialport::SerialPort>,
	program: Vec<Vec<String>>,
}
impl CncEngine {
	pub fn new(device: &str) -> Result<Self, CncError> {
		println!("Initializing engine");
		let port = match serialport::new(device, 115200)
			.timeout(std::time::Duration::from_millis(100))
			.open()
		{
			Ok(result) => {
				println!("Connection created");
				result
			}
			Err(err) => {
				eprintln!("Selected device not available.");
				eprintln!("Please choose one of the available devices:");
				Self::print_available_devices()?;
				return Err(err.into());
			}
		};
		let mut ret_engine = Self {
			curr_location: (0.0, 0.0, 0.0),
			initialized: false,
			port,
			program: vec![],
		};
		println!("Initial homing");
		ret_engine.initialized = true;
		ret_engine.home();
		ret_engine.stop_motors();
		Ok(ret_engine)
	}

	fn check_initialization(&self) -> Result<(), CncError> {
		println!("Check initialization");
		if self.initialized {
			return Ok(());
		}
		return Err("Engine not initialized. You must home before starting".into());
	}

	pub fn print_available_devices() -> Result<(), CncError> {
		let available_devices = serialport::available_ports()?;
		for device in available_devices {
			println!(" - {}", device.port_name);
		}
		Ok(())
	}

	pub fn get_curr_location(&self) -> Point2 {
		return (self.curr_location.0, self.curr_location.1).into();
	}

	pub fn home(&mut self) -> Result<(), CncError> {
		println!("Homing");
		self.execute_command("G28")
	}

	pub fn execute_routine(&mut self, command_vec: &Vec<String>) -> Result<(), CncError> {
		println!("Commands: `{:?}`", command_vec);
		self.check_initialization()?;
		for command in command_vec {
			self.execute_command(&command)?;
		}
		Ok(())
	}

	pub fn execute_programm(&mut self) -> Result<(), CncError> {
		for routine in self.program.clone() {
			self.execute_routine(&routine)?;
		}
		self.home()?;
		self.stop_motors()?;
		Ok(())
	}

	pub fn push_routine(&mut self, routine: Vec<String>) {
		self.program.push(routine);
	}

	fn execute_command(&mut self, command: &str) -> Result<(), CncError> {
		std::thread::sleep(std::time::Duration::from_millis(20));
		println!("-> {}", &command);
		self.port.write(command.as_bytes()).unwrap();
		self.port.write(&[13u8]).unwrap();
		let mut ok = false;
		while !ok {
			let received_message = Self::get_device_response(&mut self.port);
			print!("   <- {}", &received_message);
			if received_message.ends_with("ok\n") {
				ok = true;
			}
		}
		Ok(())
	}

	fn get_device_response(port: &mut std::boxed::Box<dyn serialport::SerialPort>) -> String {
		let mut serial_buf: Vec<u8> = vec![0; 32];
		let mut response = String::new();
		let mut is_done = false;
		while !is_done {
			match port.read(&mut serial_buf) {
				Ok(num) => {
					response = format!(
						"{}{}",
						response,
						std::str::from_utf8(&serial_buf[..num]).unwrap()
					);
					if response.ends_with("\n") {
						is_done = true;
					}
				}
				Err(_) => {}
			}
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
		return response;
	}
	pub fn stop_motors(&mut self) -> Result<(), CncError> {
		self.execute_command("M84 X Y E Z")
	}
}
