
use itertools::Itertools;
use std::collections::{VecDeque, HashSet};

type Grid = Vec<Vec<u32>>;

fn get_adjacent_points((r, c): (usize, usize), grid: &Grid) -> Vec<(usize,usize)> {
	let mut result = Vec::new();
	if r > 0 { result.push((r - 1, c)); }
	if r < grid.len() - 1 { result.push((r + 1, c)); }
	if c > 0 { result.push((r, c - 1)); }
	if c < grid[0].len() - 1 { result.push((r, c + 1)); }
	return result;
}

pub fn solve(inputs: Vec<String>) {

	let grid: Grid = inputs.iter().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect()).collect();

	let mut low_points: Vec<(usize, usize)> = Vec::new();

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let adjacents = get_adjacent_points((r, c), &grid);

			if adjacents.iter().all(|&(adj_r, adj_c)| grid[adj_r][adj_c] > grid[r][c]) {
				low_points.push((r, c));
			}
		}
	}

	let mut basin_sizes = Vec::new();

	for low_point in &low_points {
		let mut q: VecDeque<(usize, usize)> = VecDeque::new();

		let mut visited = HashSet::new();
		q.push_back((low_point.0, low_point.1));

		while !q.is_empty() {
			let (r, c) = q.pop_front().unwrap();

			if grid[r][c] == 9 {
				continue;
			}

			if !visited.insert((r, c)) {
				continue;
			}

			for adjacent in get_adjacent_points((r, c), &grid) {
				q.push_back(adjacent);
			}
		}

		basin_sizes.push(visited.len());
	}

    println!("Part 1: {}", low_points.iter().map(|&(r, c)| grid[r][c] + 1).sum::<u32>());
    println!("Part 2: {}", basin_sizes.iter().sorted().rev().take(3).product());
}