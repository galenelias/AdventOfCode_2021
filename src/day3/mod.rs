use itertools::Itertools;

fn most_common_char(inputs: &Vec<Vec<char>>, index: usize) -> char {
	let count_ones = inputs.iter().filter(|line| line[index] == '1').count();
	if count_ones >= (inputs.len() + 1) / 2 {
		'1'
	} else {
		'0'
	}
}

fn opposite_char(ch: char) -> char {
	if ch == '0' { '1' } else { '0' }
}

fn part1(inputs: &Vec<Vec<char>>) {
	let mut gamma_str = String::new();
	let mut epsilon_str = String::new();

	for i in 0..inputs[0].len() {
		let common_char = most_common_char(inputs, i);
		gamma_str.push(common_char);
		epsilon_str.push(opposite_char(common_char));
	}

	println!("Part 1: {}", u64::from_str_radix(&gamma_str, 2).unwrap() * u64::from_str_radix(&epsilon_str, 2).unwrap());
}

fn filter_nums(inputs: &mut Vec<Vec<char>>, i: usize, keep_most_common: bool) {
	if inputs.len() > 1 {
		let common_char = most_common_char(inputs, i);
		let filter_char = if keep_most_common { common_char } else { opposite_char(common_char) };
		inputs.retain(|line| line[i] == filter_char);
	}
}

fn part2(inputs: &Vec<Vec<char>>) {
	let digit_count = inputs[0].len();

	let mut oxygen_numbers = inputs.clone();
	let mut co2_numbers = inputs.clone();

	for i in 0..digit_count {
		filter_nums(&mut oxygen_numbers, i, true /*keep_most_common*/);
		filter_nums(&mut co2_numbers, i, false /*keep_most_common*/);
	}

	// Convert characters back into a string, then do a radix parse
	let binary_vec_to_int = |vec: &[char]| { u64::from_str_radix(&vec.iter().collect::<String>(), 2).unwrap() };

	println!("Part 2: {}", binary_vec_to_int(&oxygen_numbers[0]) * binary_vec_to_int(&co2_numbers[0]));
}

pub fn solve(inputs: Vec<String>) {
	let commands = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec(); // Vec<String> -> Vec<Vec<char>>

	part1(&commands);
	part2(&commands);
}