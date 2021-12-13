use std::collections::HashSet;
use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let mut grid: HashSet<(usize, usize)> = HashSet::new();

	for input in &inputs {
		if input.len() == 0 {
			break;
		}
		let coord = input.split(',').map(|s| s.parse::<usize>().unwrap()).collect_vec();
		grid.insert((coord[1], coord[0]));
	}

	for fold in inputs.iter().filter(|line| line.starts_with("fold along")) {
		let (_, val) = fold.split_once("=").unwrap();
		let fold_val = val.parse::<usize>().unwrap();

		let is_vertical = fold.starts_with("fold along x");
		let mut new_grid = HashSet::new();

		for &(r, c) in &grid {
			if is_vertical && c > fold_val {
				new_grid.insert((r, fold_val - (c - fold_val)));
			} else if !is_vertical && r > fold_val {
				new_grid.insert((fold_val - (r - fold_val), c));
			} else {
				new_grid.insert((r, c));
			}
		}

		grid = new_grid;
	}

	let max_r = grid.iter().map(|c| c.0).max().unwrap();
	let max_c = grid.iter().map(|c| c.1).max().unwrap();

	for r in 0..=max_r {
		println!("{}", (0..=max_c).map(|c| if grid.contains(&(r, c)) { '#' } else { ' ' }).collect::<String>());
	}
}

