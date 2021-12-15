use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

type Grid = Vec<Vec<usize>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct BfsNode {
	risk: usize,
	pos: (usize, usize),
}

impl Ord for BfsNode {
	fn cmp(&self, other: &Self) -> Ordering {
		self.risk.cmp(&other.risk).reverse()
	}
}

impl PartialOrd for BfsNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn get_adjacent_points((r, c): (usize, usize), grid: &Grid) -> Vec<(usize,usize)> {
	let mut result = Vec::new();
	if r > 0 { result.push((r - 1, c)); }
	if r < grid.len() - 1 { result.push((r + 1, c)); }
	if c > 0 { result.push((r, c - 1)); }
	if c < grid[0].len() - 1 { result.push((r, c + 1)); }
	return result;
}

fn enlarge_grid(grid: &Grid) -> Grid {
	let dim = grid.len();
	let mut result = vec![vec![0; dim * 5]; dim * 5];

	for rm in 0..5 {
		for cm in 0..5 {
			for r in 0..dim {
				for c in 0..dim {
					let value = grid[r][c] + rm + cm;
					result[dim * rm + r][dim * cm + c] = if value >= 10 { value - 9 } else { value };
				}
			}
		}
	}

	return result;
}

fn bfs_grid(grid: &Grid) -> usize {
	let mut visited = HashSet::new();
	let mut heap = BinaryHeap::new();

	let end_pos = (grid.len() - 1, grid[0].len() - 1);

	heap.push(BfsNode{risk: 0, pos: (0, 0)});

	while !heap.is_empty() {
		let node = heap.pop().unwrap();

		if node.pos == end_pos {
			return node.risk;
		}

		if !visited.insert(node.pos) {
			continue;
		}

		for adj in get_adjacent_points((node.pos.0, node.pos.1), &grid) {
			heap.push(BfsNode{ risk: node.risk + grid[adj.0][adj.1], pos: adj});
		}
	}

	unreachable!();
}

pub fn solve(inputs: Vec<String>) {
	let grid: Grid = inputs.iter().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()).collect();
	println!("Part 1: {}", bfs_grid(&grid));

	let large_grid = enlarge_grid(&grid);
	println!("Part 2: {}", bfs_grid(&large_grid));
}
