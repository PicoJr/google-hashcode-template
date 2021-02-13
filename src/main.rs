#[macro_use]
extern crate clap;

use log::info;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::data::{output_as_lines, parse_input, solve};

mod cli;
mod data;
mod utils;

fn main() -> anyhow::Result<()> {
    // cf https://crates.io/crates/env_logger
    env_logger::init();

    // parse command line arguments
    let matches = cli::get_app().get_matches();
    let input_files = matches.values_of("input");
    let dump = !matches.is_present("dry");
    let output_directory = matches.value_of("out").unwrap_or(".");
    if let Some(input_files) = input_files {
        for (i, input_file_path) in input_files.enumerate() {
            let path = PathBuf::from_str(input_file_path)?;
            let input_content = read_to_string(path)?;
            let input_data = parse_input(input_content.as_str())?;
            let (output_data, score_maybe) = solve(&input_data)?;
            if let Some(score) = score_maybe {
                info!("{} score: {}", input_file_path, score);
            }
            if dump {
                let dump_lines = output_as_lines(&output_data)?;
                let output_file_name = format!("output_{}.out", i);
                let output_path = Path::new(output_directory).join(PathBuf::from(output_file_name));
                info!("writing to {:?}", output_path);
                let mut output_file = File::create(output_path)?;
                for line in dump_lines {
                    writeln!(output_file, "{}", line)?;
                }
            }
        }
    }
    Ok(())
}
