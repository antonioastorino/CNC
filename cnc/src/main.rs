mod cnc_commands;
mod cnc_error;
mod cnc_geometry;
mod cnc_settings;
use cnc_geometry::*;

fn main() -> Result<(), cnc_error::CncError> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Please choose one of the available devices:");
        cnc_commands::CncEngine::print_available_devices()?;
        return Err("Missing argument: device".into());
    }
    let mut cnc = cnc_commands::CncEngine::new(&args.collect::<Vec<String>>()[1])?;
    // Square
    let mut polygon_vec: Vec<Polygon> = vec![];
    for sides in 3..4 {
        polygon_vec.push(Polygon::new_with_side_length(
            (100.0, 150.0).into(),
            15.0,
            sides,
            0.0,
        ));
    }
    
    for polygon in polygon_vec {
         cnc.push_routine(polygon.to_commands());
    }

    cnc.execute_programm()?;

    Ok(())
}
