/// Really simple CLI app to copy Mac OS Calendar.app calendars sources to other location
/// e.g. `cpcalendars <destination>`

use std::fs;

use fs_extra::{copy_items, dir};
use glob::glob;
use structopt::StructOpt;

const SRC_ROOT: &'static str = "Library/Calendars";

#[derive(StructOpt)]
struct Cli {
    /// The destination path where copy the calendars
    #[structopt(parse(from_os_str))]
    destination: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let destination = args.destination;

    // Calculate final destination path and create the directory
    let composed_destination = destination.join("All Calendars.icbu");
    match fs::create_dir(&composed_destination) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(f) => f,
    }

    // Create a vector to save the calendar paths
    let mut calendars = Vec::new();

    // Expand home directory
    let src_root = dirs::home_dir().unwrap().join(&SRC_ROOT);

    // Push "*.caldav" calendars
    for e in
        glob(&src_root.join("*.caldav").to_str().unwrap()).expect("Failed to read glob pattern")
    {
        calendars.push(e.unwrap());
    }
    // Push "*.calendar" calendars
    for e in
        glob(&src_root.join("*.calendar").to_str().unwrap()).expect("Failed to read glob pattern")
    {
        calendars.push(e.unwrap());
    }
    // Push "*.exchange" calendars
    for e in
        glob(&src_root.join("*.exchange").to_str().unwrap()).expect("Failed to read glob pattern")
    {
        calendars.push(e.unwrap());
    }

    // Copy all the calendars to the final destination
    let options = dir::CopyOptions::new(); //Default options
    match copy_items(&calendars, composed_destination, &options) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(_f) => println!("Ok, all calendars have been copied."),
    }
}
