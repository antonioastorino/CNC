use std::io::Write;
mod cnc_commands;
mod cnc_error;
mod cnc_geometry;
mod cnc_settings;
use cnc_geometry::*;

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

fn print_available_devices() -> Result<(), cnc_error::CncError> {
    let available_devices = serialport::available_ports()?;
    for device in available_devices {
        println!(" - {}", device.port_name);
    }
    Ok(())
}

fn main() -> Result<(), cnc_error::CncError> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Please choose one of the available devices:");
        print_available_devices()?;
        return Err("Missing argument: device".into());
    }
    let mut port = match serialport::new(&args.collect::<Vec<String>>()[1], 115200)
        .timeout(std::time::Duration::from_millis(100))
        .open()
    {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Selected device not available.");
            eprintln!("Please choose one of the available devices:");
            print_available_devices()?;
            return Err(err.into());
        }
    };
    // Square
    let mut polygon_vec: Vec<Polygon> = vec![];
    for sides in 3..20 {
        polygon_vec.push(Polygon::new_with_side_length(
            (100.0, 150.0).into(),
            15.0,
            sides,
            0.0,
        ));
    }
    let mut program: Vec<Vec<String>> = vec![cnc_commands::home()];
    for i in 0..polygon_vec.len() {
        program.push(polygon_vec[i].to_commands());
    }

    program.push(cnc_commands::home());
    program.push(cnc_commands::stop_motors());

    for command_vec in program {
        for command in command_vec {
            std::thread::sleep(std::time::Duration::from_millis(20));
            println!("-> {}", &command);
            port.write(command.as_bytes()).unwrap();
            port.write(&[13u8]).unwrap();
            let mut ok = false;
            while !ok {
                let received_message = get_device_response(&mut port);
                print!("   <- {}", &received_message);
                if received_message.ends_with("ok\n") {
                    ok = true;
                }
            }
        }
    }
    Ok(())
}
