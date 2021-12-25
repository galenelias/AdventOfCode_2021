use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Variable {
	W, X, Y, Z
}

fn index_of(v: &Variable) -> usize {
	match v {
		Variable::W => 0,
		Variable::X => 1,
		Variable::Y => 2,
		Variable::Z => 3,
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum VarOrNum {
	Variable(Variable),
	Number(i64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
	Input(Variable),
	Add(Variable, VarOrNum),
	Mul(Variable, VarOrNum),
	Div(Variable, VarOrNum),
	Mod(Variable, VarOrNum),
	Eql(Variable, VarOrNum),
}

fn parse_variable(token: &str) -> Variable {
	match token { 
		"w" => Variable::W,
		"x" => Variable::X,
		"y" => Variable::Y,
		"z" => Variable::Z,
		_ => unreachable!("Unexpected token"),
	}
}

fn parse_variable_or_number(token: &str) -> VarOrNum {
	match token {
		"w" | "x" | "y" | "z" => VarOrNum::Variable(parse_variable(token)),
		_ => VarOrNum::Number(token.parse::<i64>().unwrap()),
	}
}

fn parse_program(inputs: &Vec<String>) -> Vec<Instruction> {
	inputs.iter().map(|line| {
		let parts = line.split_whitespace().collect_vec();
		match parts[0] {
			"inp" => Instruction::Input(parse_variable(parts[1])),
			"add" => Instruction::Add(parse_variable(parts[1]), parse_variable_or_number(parts[2])),
			"mul" => Instruction::Mul(parse_variable(parts[1]), parse_variable_or_number(parts[2])),
			"div" => Instruction::Div(parse_variable(parts[1]), parse_variable_or_number(parts[2])),
			"mod" => Instruction::Mod(parse_variable(parts[1]), parse_variable_or_number(parts[2])),
			"eql" => Instruction::Eql(parse_variable(parts[1]), parse_variable_or_number(parts[2])),
			_ => unreachable!("Unexpected insruction: {}", parts[0]),
		}
	}).collect()
}

fn run(program: &[Instruction], inputs: &[i64]) -> [i64; 4] {
	let mut regs = [0i64; 4];
	let mut input_offset = 0;

	let get_val = |v: &VarOrNum, regs: &[i64]| -> i64 {
		match v {
			VarOrNum::Number(v) => *v,
			VarOrNum::Variable(v) => regs[index_of(v)],
		}
	};

	for instr in program {
		match instr {
			Instruction::Input(r) => { regs[index_of(r)] = inputs[input_offset]; input_offset += 1; },
			Instruction::Add(r, v) => regs[index_of(r)] += get_val(v, &regs),
			Instruction::Mul(r, v) => regs[index_of(r)] *= get_val(v, &regs),
			Instruction::Div(r, v) => regs[index_of(r)] /= get_val(v, &regs),
			Instruction::Mod(r, v) => regs[index_of(r)] %= get_val(v, &regs),
			Instruction::Eql(r, v) => regs[index_of(r)] = (regs[index_of(r)] == get_val(v, &regs)) as i64,
		}
	}

	return regs;
}

fn sub_solve(program: &[Instruction], find_highest: bool) -> i64
{
	let blocks = program.chunks(18).collect_vec();
	let mut digits = [0; 14];
	let mut digit_stack = Vec::new();

	// Pair the 'push' blocks with the 'pop' blocks using a Vec as a stack.  When we find a pair, run just
	// that pair to find the two digits which maximize our highest/lowest condition while cancelling out
	for (i, block) in blocks.iter().enumerate() {
		let is_push = block.iter().any(|instr| instr == &Instruction::Div(Variable::Z, VarOrNum::Number(1)));
		if is_push {
			digit_stack.push(i);
		} else {
			let first_block = digit_stack.pop().unwrap();
			let sub_program = blocks[first_block].iter().chain(blocks[i].iter()).copied().collect_vec();

			let digit_range: Vec<i64> = if find_highest { (1..=9).rev().collect() } else { (1..=9).collect() };

			'outer: for &first in &digit_range {
				for &second in &digit_range {
					let input = [first, second];
					let result = run(&sub_program, &input);
					if result[index_of(&Variable::Z)] == 0 {
						digits[first_block] = first;
						digits[i] = second;
						break 'outer;
					}
				}
			}
		}
	}

	assert!(!digits.iter().any(|&d| d == 0));

	digits.iter().fold(0, |a, b| a * 10 + b)
}

pub fn solve(inputs: Vec<String>) {

	let program = parse_program(&inputs);

	let part1 = sub_solve(&program, true /*highest*/);
	println!("Part 1: {}", part1);

	let part2 = sub_solve(&program, false /*highest*/);
	println!("Part 2: {}", part2);
}