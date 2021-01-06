use super::cnc_settings::*;
use std::f32::consts::PI;

pub trait ToCommands {
	fn to_commands(&self) -> Vec<String>;
}

#[derive(Clone)]
pub enum GParameter {
	X(f32),
	Y(f32),
	Z(f32),
	F(f32),
}

impl GParameter {
	fn get_variant(&self) -> char {
		match self {
			Self::X(_) => 'X',
			Self::Y(_) => 'Y',
			Self::Z(_) => 'Z',
			Self::F(_) => 'F',
		}
	}
	fn get_value(&self) -> f32 {
		match self {
			Self::X(value) => *value,
			Self::Y(value) => *value,
			Self::Z(value) => *value,
			Self::F(value) => *value,
		}
	}

	fn error_mismatched_type(&self, other: &Self) -> Result<(), super::cnc_error::CncError> {
		if self.get_variant() != other.get_variant() {
			return Err(format!(
				"Coordinate lable mismatch: `{}` != `{}`",
				self.get_variant(),
				other.get_variant()
			)
			.into());
		}
		Ok(())
	}

	fn is_less_than(&self, other: &Self) -> Result<bool, super::cnc_error::CncError> {
		self.error_mismatched_type(other)?;
		return Ok(self.get_value() < other.get_value());
	}

	fn is_greater_than(&self, other: &Self) -> Result<bool, super::cnc_error::CncError> {
		self.error_mismatched_type(other)?;
		return Ok(self.get_value() > other.get_value());
	}

	fn is_invalid(&self) -> Result<bool, super::cnc_error::CncError> {
		let min_val: Self;
		let max_val: Self;
		match self {
			Self::X(_) => {
				min_val = min('X')?;
				max_val = max('X')?;
			}
			Self::Y(_) => {
				min_val = min('Y')?;
				max_val = max('Y')?;
			}
			Self::Z(_) => {
				min_val = min('Z')?;
				max_val = max('Z')?;
			}
			Self::F(_) => {
				min_val = min('F')?;
				max_val = max('F')?;
			}
		}
		return Ok(self.is_less_than(&min_val)? || self.is_greater_than(&max_val)?);
	}
}

impl std::fmt::Display for GParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.is_invalid() {
			Ok(value) => {
				if value {
					panic!("Out of boundaries");
				} else {
					write!(f, "{}{:.3}", self.get_variant(), self.get_value())
				}
			}
			Err(err) => panic!("{}", err),
		}
	}
}

#[derive(Clone)]
pub struct Point2 {
	x: GParameter,
	y: GParameter,
}

impl std::ops::Add for Point2 {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self {
			x: GParameter::X(self.x.get_value() + other.x.get_value()),
			y: GParameter::Y(self.y.get_value() + other.y.get_value()),
		}
	}
}

impl Point2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self {
			x: GParameter::X(x),
			y: GParameter::Y(y),
		}
	}
}

impl From<(f32, f32)> for Point2 {
	fn from(tuple: (f32, f32)) -> Self {
		Self {
			x: GParameter::X(tuple.0),
			y: GParameter::Y(tuple.1),
		}
	}
}

impl std::fmt::Display for Point2 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {}", self.x, self.y)
	}
}

pub struct Segment {
	start: Point2,
	stop: Point2,
}

impl Segment {
	pub fn new(start: Point2, stop: Point2) -> Self {
		Self { start, stop }
	}
}

impl ToCommands for Segment {
	fn to_commands(&self) -> Vec<String> {
		return [
			// Go up fast
			format!("G1 {} {}", move_speed(), move_height()),
			// Go to start
			format!("G1 {}", self.start),
			// Go to touch height
			format!("G1 {}", touch_height()),
			// Drill slowly
			format!("G1 {} {}", mill_speed(), mill_height()),
			// Mill to stop
			format!("G1 {}", self.stop),
			// Go up fast
			format!("G1 {} {}", move_speed(), move_height()),
		]
		.iter()
		.cloned()
		.collect();
	}
}

pub struct Polygon {
	center: Point2,
	radius: f32,
	num_of_sides: u8,
	theta_start: f32, // Start angle in degrees
}

impl Polygon {
	pub fn new_with_circumradius(
		center: Point2,
		radius: f32,
		num_of_sides: u8,
		theta_start: f32,
	) -> Self {
		Polygon {
			center,
			radius,
			num_of_sides,
			theta_start,
		}
	}
	pub fn new_with_side_length(
		center: Point2,
		side_length: f32,
		num_of_sides: u8,
		theta_start: f32,
	) -> Self {
		let radius = side_length / (PI / num_of_sides as f32).sin() / 2.0;
		Polygon {
			center,
			radius,
			num_of_sides,
			theta_start,
		}
	}
}

impl ToCommands for Polygon {
	fn to_commands(&self) -> Vec<String> {
		let theta_start_rad = self.theta_start * PI / 180.0
			+ PI * (1.0 / (self.num_of_sides as f32) - 0.5);
		// Unless differently specified, always start from the bottom-right corner.
		let start = self.center.clone()
			+ (
				self.radius * theta_start_rad.cos(),
				self.radius * theta_start_rad.sin(),
			)
				.into();
		let mut command_vec: Vec<String> = vec![];
		command_vec.push(
			// Go up fast
			format!("G1 {} {}", move_speed(), move_height()),
		);
		command_vec.push(
			// Go to start
			format!("G1 {}", start),
		);
		command_vec.push(
			// Go to touch height
			format!("G1 {}", touch_height()),
		);
		command_vec.push(
			// Drill slowly
			format!("G1 {} {}", mill_speed(), mill_height()),
		);
		let step_theta = 2.0 * PI / self.num_of_sides as f32;
		for i in 0..=self.num_of_sides {
			let theta = i as f32 * step_theta + theta_start_rad;
			let curr_point =
				self.center.clone() + (self.radius * theta.cos(), self.radius * theta.sin()).into();

			command_vec.push(format!("G1 {}", curr_point));
		}
		command_vec.push(
			// Go up fast
			format!("G1 {} {}", move_speed(), move_height()),
		);
		return command_vec;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_display_code() {
		let x = GParameter::X(45.0);
		let y = GParameter::Y(12.345);
		let z = GParameter::Z(15.3);
		let speed = GParameter::F(1592.42554);
		assert_eq!(format!("{}", x), "X45.000");
		assert_eq!(format!("{}", y), "Y12.345");
		assert_eq!(format!("{}", z), "Z15.300");
		assert_eq!(format!("{}", speed), "F1592.426");
	}

	#[test]
	fn test_segment_to_commands() {
		let start = Point2::new(10.5, 20.5);
		let stop = Point2::new(20.5, 10.5);
		let segment = Segment::new(start.clone(), stop.clone());
		let commands = segment.to_commands();
		assert_eq!(
			commands[0],
			format!("G1 {} {}", move_speed(), move_height())
		);
		assert_eq!(commands[1], format!("G1 {}", start));
		assert_eq!(commands[2], format!("G1 {}", touch_height()));
		assert_eq!(
			commands[3],
			format!("G1 {} {}", mill_speed(), mill_height())
		);
		assert_eq!(commands[4], format!("G1 {}", stop));
		assert_eq!(
			commands[5],
			format!("G1 {} {}", move_speed(), move_height())
		);
	}
}
