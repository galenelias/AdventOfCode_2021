fn get_closing(ch: char) -> char {
	match ch {
		'(' => ')',
		'[' => ']',
		'{' => '}',
		'<' => '>',
		_ => unreachable!(),
	}
}

pub fn solve(inputs: Vec<String>) {
	let inputs: Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect()).collect();

	let mut part1 = 0;
	let mut part2_scores = Vec::new();

	for line in inputs {
		let mut stack = Vec::new();
		let mut valid = true;

		for ch in line {
			match ch {
				'(' | '[' | '{' | '<' => { stack.push(ch); }
				')' | ']' | '}' | '>' => {
					let opener = stack.pop().unwrap();
					let expected = get_closing(opener);

					if ch != expected {
						part1 += match ch {
							')' => 3,
							']' => 57,
							'}' => 1197,
							'>' => 25137,
							_ => unreachable!(),
						};
						valid = false;
						break;
					}
				}
				_ => unreachable!(),
			}
		}

		if valid {
			let mut part2_score: u64 = 0;

			for ch in stack.iter().map(|&ch| get_closing(ch)).rev() {
				part2_score *= 5;
				part2_score += match ch {
					')' => 1,
					']' => 2,
					'}' => 3,
					'>' => 4,
					_ => unreachable!(),
				};
			}
			part2_scores.push(part2_score);
		}
	}

	part2_scores.sort();

	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2_scores[part2_scores.len() / 2]);
}