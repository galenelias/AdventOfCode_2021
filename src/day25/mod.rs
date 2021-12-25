use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	for i in 0.. {
		let original_grid = grid.clone();
		let mut new_grid = grid.clone();

		// First move the '>'s
		for r in 0..grid.len() {
			for c in 0..grid[r].len() {
				let ch = grid[r][c];
				if ch == '>' {
					let new_c = (c + 1) % grid[r].len();
					if grid[r][new_c] == '.' {
						new_grid[r][new_c] = '>';
						new_grid[r][c] = '.';
					}
				}
			}
		}

		grid = new_grid.clone();

		// Now the 'v's
		for r in 0..grid.len() {
			for c in 0..grid[r].len() {
				let ch = grid[r][c];
				if ch == 'v' {
					let new_r = (r + 1) % grid.len();
					if grid[new_r][c] == '.' {
						new_grid[new_r][c] = 'v';
						new_grid[r][c] = '.';
					}
				}
			}
		}

		grid = new_grid;

		if grid == original_grid {
			println!("Part 1: {}", i + 1);
			break;
		}
	}
}