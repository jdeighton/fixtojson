use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use fixparser::FixMessage;
use flate2::read::GzDecoder;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(long_help = "File(s) containing one FIX message per line to be converted to JSONL format", default_value = "-")]
	files: Vec<String>,
	
}

#[derive(Debug)]
pub struct Config {
	files: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
	let args = Args::parse();
	Ok(Config {
		files: args.files,
	})
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
	match filename {
		"-" => Ok(Box::new(BufReader::new(io::stdin()))),
		_ => {
			let path = Path::new(filename);
			if path.extension() == Some(OsStr::new("gz")) {
				Ok(Box::new(BufReader::new(GzDecoder::new(File::open(&path)?))))
			} else {
				Ok(Box::new(BufReader::new(File::open(filename)?)))
			}
		}
	}
}

pub fn run(config: Config) -> MyResult<()> {
	for filename in config.files {
		match open(&filename) {
			Err(err) => eprintln!("Failed to open {}: {}", filename, err),
			Ok(file) => {
				for (line_num, line_result) in file.lines().enumerate() {
					let line = line_result?;	
					if let Some(fix_message) = FixMessage::from_tag_value(&line) {
						println!("{}", fix_message.to_json());	
					}
					else {
						println!("Could not parse FIX message from at line {}: {}",line_num,  &line)
					}
				}
			},
		}
	}
	Ok(())
}