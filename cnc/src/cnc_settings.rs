pub fn min(label: char) -> Result<super::cnc_geometry::GParameter, super::cnc_error::CncError> {
	match label {
		'X' => Ok(super::cnc_geometry::GParameter::X(10.0)),
		'Y' => Ok(super::cnc_geometry::GParameter::Y(10.0)),
		'Z' => Ok(super::cnc_geometry::GParameter::Z(0.01)),
		'F' => Ok(super::cnc_geometry::GParameter::F(1.0)),
		_ => Err(format!("Invalid parameter label `{}`", label).into()),
	}
}

pub fn max(label: char) -> Result<super::cnc_geometry::GParameter, super::cnc_error::CncError> {
	match label {
		'X' => Ok(super::cnc_geometry::GParameter::X(200.0)),
		'Y' => Ok(super::cnc_geometry::GParameter::Y(200.0)),
		'Z' => Ok(super::cnc_geometry::GParameter::Z(240.0)),
		'F' => Ok(super::cnc_geometry::GParameter::F(6000.0)),
		_ => Err(format!("Invalid parameter label `{}`", label).into()),
	}
}

pub fn move_speed() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::F(6000.0);
}
pub fn mill_speed() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::F(200.0);
}
pub fn move_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(20.0);
}
pub fn touch_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(10.0);
}
pub fn mill_height() -> super::cnc_geometry::GParameter {
	return super::cnc_geometry::GParameter::Z(5.0);
}
