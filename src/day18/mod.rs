use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum SnailChar {
	OpenBrace,
	CloseBrace,
	Comma,
	Number(i64),
}

fn _to_string(chars: &[SnailChar]) -> String {
	chars
		.iter()
		.map(|ch| match ch {
			SnailChar::OpenBrace => "[".to_owned(),
			SnailChar::CloseBrace => "]".to_owned(),
			SnailChar::Comma => ",".to_owned(),
			SnailChar::Number(v) => format!("{}", v),
		})
		.collect::<String>()
}

fn reduce(mut line: Vec<SnailChar>) -> Vec<SnailChar> {
	while reduce_explode(&mut line) || reduce_split(&mut line) {
	}
	return line;
}

fn reduce_explode(line: &mut Vec<SnailChar>) -> bool {
	let mut depth = 0;

	for (i, ch) in line.iter().enumerate() {
		match ch {
			SnailChar::OpenBrace => {
				depth += 1;
			}
			SnailChar::CloseBrace => {
				depth -= 1;
			}
			SnailChar::Comma => (),
			SnailChar::Number(val) => {
				if depth >= 5
					&& matches!(line[i - 1], SnailChar::OpenBrace)
					&& matches!(line[i + 1], SnailChar::Comma)
					&& matches!(line[i + 2], SnailChar::Number(_))
				{
					// Explode
					let lhs = val;
					let rhs = match line[i + 2] {
						SnailChar::Number(v) => v,
						_ => unreachable!(),
					};

					// Check left
					for j in (0..i - 1).rev() {
						match line[j] {
							SnailChar::Number(v) => {
								line[j] = SnailChar::Number(v + lhs);
								break;
							}
							_ => (),
						}
					}

					// Check right
					for j in i + 4..line.len() {
						match line[j] {
							SnailChar::Number(v) => {
								line[j] = SnailChar::Number(v + rhs);
								break;
							}
							_ => (),
						}
					}

					line.splice(i - 1..i + 4, [SnailChar::Number(0)]);

					return true;
				}
			}
		}
	}
	return false;
}

fn reduce_split(line: &mut Vec<SnailChar>) -> bool {
	for (i, ch) in line.iter().enumerate() {
		match ch {
			SnailChar::Number(val) => {
				if val >= &10 {
					let lhs = val / 2;
					let rhs = (val + 1) / 2;

					let _ = line.splice(
						i..i + 1,
						[
							SnailChar::OpenBrace,
							SnailChar::Number(lhs),
							SnailChar::Comma,
							SnailChar::Number(rhs),
							SnailChar::CloseBrace,
						],
					);
					return true;
				}
			}
			_ => (),
		}
	}

	return false;
}


fn concat(lhs: &Vec<SnailChar>, rhs: &Vec<SnailChar>) -> Vec<SnailChar> {
	let mut result = Vec::new();
	result.push(SnailChar::OpenBrace);
	result.extend(lhs.into_iter());
	result.push(SnailChar::Comma);
	result.extend(rhs.into_iter());
	result.push(SnailChar::CloseBrace);

	return result;
}

fn parse(line: &str) -> Vec<SnailChar> {
	line.chars()
		.map(|ch| match ch {
			'[' => SnailChar::OpenBrace,
			']' => SnailChar::CloseBrace,
			',' => SnailChar::Comma,
			_ => SnailChar::Number(ch.to_digit(10).unwrap() as i64),
		})
		.collect_vec()
}

#[derive(Debug)]
struct SnailPair {
	left: SnailElement,
	right: SnailElement,
}

#[derive(Debug)]
enum SnailElement {
	Literal(i64),
	Pair(Box<SnailPair>),
}

fn parse_tree(input: &[SnailChar]) -> SnailElement {
	let mut dummy = 0;
	parse_tree_inner(input, &mut dummy)
}

fn parse_tree_inner(input: &[SnailChar], i: &mut usize) -> SnailElement {
	let ch = input[*i];

	match ch {
		SnailChar::OpenBrace => {
			// Pair
			*i += 1;

			let left = parse_tree_inner(input, i);
			assert!(matches!(input[*i], SnailChar::Comma));
			*i += 1;

			let right = parse_tree_inner(input, i);
			assert!(matches!(input[*i], SnailChar::CloseBrace));
			*i += 1;

			SnailElement::Pair(Box::new(SnailPair { left, right }))
		},
		SnailChar::Number(val) => {
			*i += 1;
			SnailElement::Literal(val)
		}
		_ => unreachable!("Unexpected char: {:?}", ch),
	}
}

fn magnitude(ele: &SnailElement) -> i64 {
	match ele {
		SnailElement::Literal(val) => *val,
		SnailElement::Pair(pair) => 3 * magnitude(&pair.left) + 2 * magnitude(&pair.right),
	}
}

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs.iter().map(|line| parse(&line)).collect_vec();

	let mut sum = Vec::new();

	for line in inputs.iter().cloned() {
		let line = reduce(line);

		if sum.is_empty() {
			sum = line;
		} else {
			sum = concat(&sum, &line);
		}

		sum = reduce(sum);
	}

	println!("Part 1: {}", magnitude(&parse_tree(&sum)));

	let mut part2 = 0;
	for i in 0..inputs.len() {
		for j in 0..inputs.len() {
			if i == j {
				continue;
			}
			let addition = reduce(concat(&inputs[i], &inputs[j]));
			let sum = magnitude(&parse_tree(&addition));

			part2 = std::cmp::max(part2, sum);
		}
	}

	println!("Part 2: {}", part2);
}
