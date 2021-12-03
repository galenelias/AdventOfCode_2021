// #[macro_use] extern crate lazy_static;
extern crate clap;
extern crate itertools;
extern crate regex;
extern crate num;

use clap::{Arg,App};
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::fs::File;

mod day1;
mod day2;
mod day3;

fn main() {
	let matches = App::new("Advent of Code")
		.author("Galen Elias, gelias@gmail.com")
		.version("0.1.0")
		.about("Advent of code solutions in Rust")
		.arg(
			Arg::with_name("day")
				.short("d")
				.required(true)
				.index(1)
				.help("specifies which day's challenge to run")
				.validator(|str|
					str.parse::<u32>()
						.or(Err("day must be an integer".to_owned()))
						.and_then(|v| match v {
							0..=25 => Ok(()),
							_ => Err("day must be between 1 and 25".to_owned())
						})))
		.arg(
			Arg::with_name("file")
				.short("f")
				.takes_value(true)
				.help("Uses a file instead of reading from standard in"))
		.after_help("Longer explaination to appear after the options when \
					displaying the help information from --help or -h")
		.get_matches();

	let input;
	if matches.is_present("file") {
		let f = File::open(matches.value_of("file").unwrap()).unwrap();
		let file = BufReader::new(&f);
		input = file.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let stdin = io::stdin();
		input = stdin.lock().lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	}

	let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();
	match day {
		1 => day1::solve(input),
		2 => day2::solve(input),
		3 => day3::solve(input),
		_ => println!("Oops! Day {} isn't implemented yet!", day)
	}
}
