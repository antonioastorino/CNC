
pub fn home() -> Vec<String> {
	return vec!["G28".to_owned()];
}

pub fn stop_motors() -> Vec<String> {
	return vec!["M84 X Y E Z".to_owned()];
}

pub fn z_at(pos_z: f32) -> Vec<String> {
	return vec![format!("G1 Z{}", pos_z)];
}