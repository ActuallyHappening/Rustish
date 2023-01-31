use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	 /// Input file path
	 input: PathBuf,

	 /// Output file path
	 output: Option<PathBuf>,
}

fn main() {
	 let args = Args::parse();

	 // debug print args
	 println!("{:?}", args);
}