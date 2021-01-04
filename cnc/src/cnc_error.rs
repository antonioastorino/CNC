pub struct CncError {
	message: String,
}

impl std::fmt::Display for CncError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::fmt::Debug for CncError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl From<serialport::Error> for CncError {
	fn from(err: serialport::Error) -> Self {
		Self {
			message: err.description,
		}
	}
}

impl From<&str> for CncError {
	fn from(s: &str) -> Self {
		Self {
			message: s.to_owned(),
		}
	}
}
impl From<String> for CncError {
	fn from(s: String) -> Self {
		Self {
			message: s
		}
	}
}
