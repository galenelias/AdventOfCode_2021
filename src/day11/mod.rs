
use std::collections::{VecDeque};

type Grid = Vec<Vec<u32>>;

fn get_adjacent_points((r, c): (usize, usize), grid: &Grid) -> Vec<(usize,usize)> {
	let mut result = Vec::new();
	for dr in [-1, 0, 1] {
		for dc in [-1, 0, 1] {
			if dr == 0 && dc == 0 {
				continue;
			}
			let r = r as i64 + dr;
			let c = c as i64 + dc;
			if r >= 0 && c >= 0 && r < grid.len() as i64 && c < grid[r as usize].len() as i64 {
				result.push((r as usize, c as usize));
			}
		}
	}
	return result;
}

pub fn solve(inputs: Vec<String>) {
	let mut grid: Grid = inputs.iter().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect()).collect();
	let mut flashes = 0;

	for i in 0.. {
		let mut q: VecDeque<(usize, usize)> = VecDeque::new();
		let mut did_flash = vec![vec![false; grid[0].len()]; grid.len()];

		for r in 0..grid.len() {
			for c in 0..grid[r].len() {
				grid[r][c] += 1;
				if grid[r][c] == 10 {
					q.push_back((r, c));
				}
			}
		}

		while !q.is_empty() {
			let (r, c) = q.pop_front().unwrap();
			if did_flash[r][c] {
				continue;
			}

			did_flash[r][c] = true;
			flashes += 1;

			for adj in get_adjacent_points((r, c), &grid) {
				grid[adj.0][adj.1] += 1;
				if grid[adj.0][adj.1] == 10 {
					q.push_back(adj);
				}
			}
		}

		for r in 0..grid.len() {
			for c in 0..grid[r].len() {
				if grid[r][c] >= 10 {
					grid[r][c] = 0;
				}
			}
		}

		if i == 99 {
			println!("Part 1: {}", flashes);
		}

		if did_flash.iter().all(|row| row.iter().all(|&v| v)) {
			println!("Part 2: {}", i + 1);
			break;
		}
	}
}
