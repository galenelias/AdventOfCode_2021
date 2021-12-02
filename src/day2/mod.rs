use itertools::Itertools;

#[derive(Debug)]
struct Cmd {
	dir: String,
	amt: u32,
}

fn part1(commands: &Vec<Cmd>) {
	let mut pos = 0;
	let mut depth = 0;

	for cmd in commands {
		match cmd.dir.as_ref() {
			"forward" => pos += cmd.amt,
			"down" => depth += cmd.amt,
			"up" => depth -= cmd.amt,
			_ => panic!("Unrecognized command: {}", cmd.dir),
		}
	}

	println!("Part 1: {}", pos * depth);
}

fn part2(commands: &Vec<Cmd>) {
	let mut pos = 0;
	let mut depth = 0;
	let mut aim = 0;

	for cmd in commands {
		match cmd.dir.as_ref() {
			"forward" => { pos += cmd.amt; depth += aim * cmd.amt; },
			"down" => aim += cmd.amt,
			"up" => aim -= cmd.amt,
			_ => panic!("Unrecognized command: {}", cmd.dir),
		}
	}

	println!("Part 2: {}", pos * depth);
}

pub fn solve(inputs: Vec<String>) {
	let commands = inputs.iter().map(|line| {
		let mut parts = line.split(" ");
		Cmd {
			dir: parts.next().unwrap().to_owned(),
			amt: parts.next().unwrap().parse::<u32>().unwrap(),
		}
	}).collect_vec();

	part1(&commands);
	part2(&commands);
}