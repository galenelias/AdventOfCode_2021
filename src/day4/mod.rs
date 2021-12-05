use itertools::Itertools;

struct BingoSquare {
	value: u32,
	marked: bool,
}

type Board = Vec<Vec<BingoSquare>>;

fn check_solved(board: &Board) -> bool {
	board.iter().any(|row| row.iter().all(|square| square.marked))
	|| (0..5).any(|col| board.iter().all(|row| row[col].marked))
}

fn sum_unmarked(board: &Board) -> u32 {
	board.iter().flatten().filter(|square| !square.marked).map(|square| square.value).sum()
}

pub fn solve(inputs: Vec<String>) {
	let draws = inputs[0].split(",").map(|s| s.parse::<u32>().unwrap()).collect_vec();

	let mut boards = inputs[2..].chunks(6).map(|chunk| {
		chunk[0..5].iter().map(|line|
			line.split_whitespace().map(|s| BingoSquare { value: s.parse::<u32>().unwrap(), marked: false }).collect_vec()
		).collect_vec()
	}).collect_vec();

	let original_size = boards.len();

	for draw in draws {
		for board in &mut boards {
			for square in board.iter_mut().flatten() {
				if square.value == draw {
					square.marked = true;
				}
			}
		}

		if let Some(winner) = boards.iter().find(|board| check_solved(board)) {
			if boards.len() == original_size {
				println!("Part 1: {}", draw * sum_unmarked(winner));
			}

			if boards.len() == 1 {
				println!("Part 2: {}", draw * sum_unmarked(&boards[0]));
			}
		}

		boards.retain(|board| !check_solved(board));

		if boards.len() == 0 {
			break;
		}
	}
}