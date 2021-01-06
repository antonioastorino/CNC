struct Constants {}
impl Constants {
	const MOVE_SPEED: f32 = 6000.0;
	const MILL_SPEED: f32 = 3000.0;
	const MOVE_HEIGHT: f32 = 10.0;
	const TOUCH_HEIGHT: f32 = 7.0;
	const MILL_HEIGHT: f32 = 6.1;
	const MIN_X: f32 = 10.0;
	const MIN_Y: f32 = 10.0;
	const MIN_Z: f32 = 0.01;
	const MIN_F: f32 = 1.0;
	const MAX_X: f32 = 220.0;
	const MAX_Y: f32 = 220.0;
	const MAX_Z: f32 = 240.0;
	const MAX_F: f32 = 6000.0;
}

pub fn min(label: char) -> Result<super::cnc_geometry::GParameter, super::cnc_error::CncError> {
	match label {
		'X' => Ok(super::cnc_geometry::GParameter::X(Constants::MIN_X)),
		'Y' => Ok(super::cnc_geometry::GParameter::Y(Constants::MIN_Y)),
		'Z' => Ok(super::cnc_geometry::GParameter::Z(Constants::MIN_Z)),
		'F' => Ok(super::cnc_geometry::GParameter::F(Constants::MIN_F)),
		_ => Err(format!("Invalid parameter label `{}`", label).into()),
	}
}

pub fn max(label: char) -> Result<super::cnc_geometry::GParameter, super::cnc_error::CncError> {
	match label {
		'X' => Ok(super::cnc_geometry::GParameter::X(Constants::MAX_X)),
		'Y' => Ok(super::cnc_geometry::GParameter::Y(Constants::MAX_Y)),
		'Z' => Ok(super::cnc_geometry::GParameter::Z(Constants::MAX_Z)),
		'F' => Ok(super::cnc_geometry::GParameter::F(Constants::MAX_F)),
		_ => Err(format!("Invalid parameter label `{}`", label).into()),
	}
}

pub fn move_speed() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::F(Constants::MOVE_SPEED);
}
pub fn mill_speed() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::F(Constants::MILL_SPEED);
}
pub fn move_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(Constants::MOVE_HEIGHT);
}
pub fn touch_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(Constants::TOUCH_HEIGHT);
}
pub fn mill_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(Constants::MILL_HEIGHT);
}
