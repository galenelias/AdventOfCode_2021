use itertools::Itertools;

fn check_solved(marks: &Vec<Vec<bool>>) -> bool {
	for i in 0..5 {
		let mut count_row = 0;
		let mut count_col = 0;

		for j in 0..5 {
			if marks[i][j] {
				count_row += 1;
			}
			if marks[j][i] {
				count_col += 1;
			}
		}
		if count_row == 5 || count_col == 5 {
			return true;
		}
	}

	return false;
}

fn sum_unmarked(board: &Vec<Vec<u32>>, marked: &Vec<Vec<bool>>) -> u32 {
	let mut sum = 0;
	for r in 0..5 {
		for c in 0..5 {
			if !marked[r][c] {
				sum += board[r][c];
			}
		}
	}
	return sum;
}

pub fn solve(inputs: Vec<String>) {
	let draws = inputs[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect_vec();

	let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
	let mut marks: Vec<Vec<Vec<bool>>> = Vec::new();

	for b in 0..(inputs.len()-2)/6 {
		let mut board: Vec<Vec<u32>> = Vec::new();
		for r in 0..5 {
			board.push(inputs[2 + b * 6 + r].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect_vec());
		}

		boards.push(board);
		marks.push(vec![vec![false; 5]; 5]);
	}

	let mut solved_boards = vec![false; boards.len()];
	let mut solved_boards_count = 0;

	let mut part1_solved = false;
	'outer: for draw in draws {
		for b in 0..boards.len() {
			for r in 0..5 {
				for c in 0..5 {
					if boards[b][r][c] == draw {
						marks[b][r][c] = true;
					}
				}
			}

			if !solved_boards[b] && check_solved(&marks[b]) {
				solved_boards[b] = true;
				solved_boards_count += 1;

				if !part1_solved {
					println!("Part 1: {}, {}, {}", draw, b, draw * sum_unmarked(&boards[b], &marks[b]));
					part1_solved = true;
				}

				if solved_boards_count == boards.len() {
					println!("Part 2: {}, {}, {}", draw, b, draw * sum_unmarked(&boards[b], &marks[b]));
					break 'outer;
				}
			}
		}
	}
}