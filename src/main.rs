extern crate hidapi;
use hidapi::HidApi;

#[macro_use]
extern crate clap;
use clap::App;

use std::time::{Instant};

mod ds4_report;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let api = HidApi::new()?;

    match find_ds4(api.devices()) {
        None => println!("No DS4 Controllers Found"),
        Some(dev) => return process_loop(&api, &matches, dev)
    }

    Ok(())
}

fn find_ds4(devices: &[hidapi::HidDeviceInfo]) -> Option<&hidapi::HidDeviceInfo>{
    const VENDOR_ID: u16 = 1356;

    for device in devices {
        if device.vendor_id == VENDOR_ID {
            return Some(device);
        }
    }
    None
}

fn process_loop(api: &hidapi::HidApi, matches: &clap::ArgMatches, dev_info: &hidapi::HidDeviceInfo) -> Result<(), Box<dyn std::error::Error>> {

    let device = dev_info.open_device(api)?;

    device.set_blocking_mode(true)?;

    let mut read_buffer = [0u8; ds4_report::REPORT_LENGTH];

    let start_time = Instant::now();

    loop {
        device.read( &mut read_buffer[..] )?;

        let report = match ds4_report::DS4Report::new(&read_buffer) {
            None => return Ok(()), // Add error code
            Some(report) => report
        };

        if matches.is_present("time") {
            print!("{:8}, ", Instant::now().duration_since(start_time).as_millis());
        }

        if matches.is_present("gyro") {
            print!("{:10.6}, {:10.6}, {:10.6}, ", report.get_gyroscope_x(), report.get_gyroscope_y(), report.get_gyroscope_z());
        }

        if matches.is_present("accel") {
            print!("{:10.6}, {:10.6}, {:10.6}, ", report.get_accelerometer_x(), report.get_accelerometer_y(), report.get_accelerometer_z());
        }

        if matches.is_present("jl") {
            let left_joystick = report.get_left_joystick();
            print!("{:4}, {:4}, ", left_joystick.x, left_joystick.y);
        }

        if matches.is_present("jr") {
            let right_joystick = report.get_right_joystick();
            print!("{:4}, {:4}, ", right_joystick.x, right_joystick.y);
        }

        if matches.is_present("trigger") {
            print!("{:4}, {:4}, ", report.get_l2_trigger_analog(), report.get_r2_trigger_analog());
        }

        if matches.is_present("bs") {
            print!("{}, {}, {}, {}, ",
                   report.get_button_triangle() as i8,
                   report.get_button_circle() as i8,
                   report.get_button_cross() as i8,
                   report.get_button_square() as i8);
        }

        if matches.is_present("bd") {
            let dpad = report.get_button_dpad();
            print!("{}, {}, {}, {}, ",
                   dpad.up as i8,
                   dpad.right as i8,
                   dpad.down as i8,
                   dpad.left as i8);
        }

        if matches.is_present("bt") {
            print!("{}, {}, {}, {}, {}, {}, ",
                   report.get_button_l1() as i8,
                   report.get_button_l2() as i8,
                   report.get_button_l3() as i8,
                   report.get_button_r1() as i8,
                   report.get_button_r2() as i8,
                   report.get_button_r3() as i8);
        }

        if matches.is_present("be") {
            print!("{}, {}, {}, {}, ",
                   report.get_button_options() as i8,
                   report.get_button_share() as i8,
                   report.get_button_touchpad() as i8,
                   report.get_button_playstation() as i8);
        }

        println!();
    }
}
