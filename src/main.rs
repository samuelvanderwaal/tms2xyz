#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::{Regex};
use std::error::Error;

fn main() {
    let matches = App::new("TMS To XYZ")
        .version(crate_version!())
        .author("Samuel J Vanderwaal")
        .about("Converts TMS style tile names to XYZ")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .takes_value(true)
                .value_name("TYPE")
                .help("Set image extension, e.g.: 'jpg'. Defaults to png"),
        )
        .arg(
            Arg::with_name("DIR")
                .help("The path to the directory to be parsed.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("dryrun")
                .short("d")
                .long("dryrun")
                .help("Performs dryrun and does not change file names."),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Display verbose output.")
        )
        .get_matches();

    let dir = Path::new(matches.value_of("DIR").unwrap());
    let img_ext = matches.value_of("type").unwrap_or("png").to_lowercase();
    let mut verbose = matches.is_present("verbose");
    let dryrun = matches.is_present("dryrun");

    if dryrun {
        verbose = true;
    }

    match parse_dir(dir, &img_ext, dryrun, verbose) {
        Ok(()) => (),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn parse_dir(dir: &Path, ext: &str, dryrun: bool, verbose: bool) -> Result<(), Box<dyn Error>> {
    let mut file_count: u32 = 0;

    if dir.is_dir() {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.path().is_file() && entry.path().extension().unwrap() == ext {
                let processed = convert_name(&entry.path(), ext, dryrun, verbose)?;

                if processed {
                    file_count += 1;
                }
            }
        }
    } else {
        println!("{:?} is not a valid directory!", dir);
        return Ok(())
    }
    println!("{:?} files processed!", file_count);
    Ok(())
}

fn convert_name(file_path: &Path, ext: &str, dryrun: bool, verbose: bool) -> Result<bool, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let mut file_renamed = false;

    let mut nums = Vec::new();
    for mat in RE.captures_iter(file_path.to_str().unwrap()) {
        let value = mat.get(0).unwrap().as_str();
        nums.push(value);
    }
    let y: i32 = nums[2].parse()?;
    let z: u32 = nums[0].parse()?;

    let new_y = 2i32.pow(z) - y - 1;

    let basename = file_path.parent().ok_or("No basename!")?;
    let new_file_name = format!("{}.{}", new_y.to_string(), ext);
    let new_path = basename.join(new_file_name);

    if verbose {
        println!("Old path: {:?}", file_path);
        println!("New path: {:?}", new_path);
    }

    if !dryrun {
        fs::rename(file_path, new_path)?;
        file_renamed = true;
    }
    Ok(file_renamed)
}