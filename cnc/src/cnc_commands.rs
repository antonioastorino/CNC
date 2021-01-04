const MIN_X: f32 = 10.0;
const MAX_X: f32 = 200.0;
const MIN_Y: f32 = 10.0;
const MAX_Y: f32 = 200.0;
const MIN_Z: f32 = 0.01;
const MAX_Z: f32 = 240.0;
use std::f32::consts::PI;

fn is_invalid_x(x: f32) -> bool {
	if x < MIN_X || x > MAX_X {
		return true;
	}
	return false;
}

fn is_invalid_y(y: f32) -> bool {
	if y < MIN_Y || y > MAX_Y {
		return true;
	}
	return false;
}

fn is_invalid_z(z: f32) -> bool {
	if z < MIN_Z || z > MAX_Z {
		return true;
	}
	return false;
}

pub fn home() -> Vec<String> {
	return vec!["G28".to_owned()];
}

pub fn stop_motors() -> Vec<String> {
	return vec!["M84 X Y E Z".to_owned()];
}

pub fn z_at(pos_z: f32) -> Vec<String> {
	return vec![format!("G1 Z{}", pos_z)];
}

pub fn circle_at(
	center_x: f32,
	center_y: f32,
	center_z: f32,
	radius: f32,
	num_of_segments: u16,
) -> Result<Vec<String>, super::cnc_error::CncError> {
	if is_invalid_x(center_x - radius)
		|| is_invalid_x(center_x + radius)
		|| is_invalid_y(center_y - radius)
		|| is_invalid_y(center_y + radius)
		|| is_invalid_z(center_z)
	{
		return Err("Circle outside boundaries".into());
	}
	let mut command_vec: Vec<String> = vec![];

	 let step_theta = 2.0 * PI / num_of_segments as f32;
	for i in 0..= num_of_segments {
		let theta = i as f32 * step_theta;
		let x = center_x + radius * theta.cos();
		let y = center_x + radius * theta.sin();
		command_vec.push(format!("G1 X{:.3} Y{:.3} Z{:.3}", x, y, center_z));
	}
	Ok(command_vec)
}
