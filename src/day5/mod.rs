use itertools::Itertools;
use std::collections::HashMap;

struct Point {
	x: i64,
	y: i64,
}

fn sub_solve(point_pairs: &Vec<Vec<Point>>, part2: bool) {
	let mut grid: HashMap<(i64, i64), usize> = HashMap::new();

	// If we're on part 1, filter down to only horizontal or vertical lines
	for points in point_pairs.iter().filter(|points| part2 || points[0].x == points[1].x || points[0].y == points[1].y) {
		let dx = (points[1].x - points[0].x).signum();
		let dy = (points[1].y - points[0].y).signum();

		let mut x = points[0].x;
		let mut y = points[0].y;

		while (x, y) != (points[1].x + dx, points[1].y + dy) {
			(*grid.entry((x, y)).or_default()) += 1;
			x += dx;
			y += dy;
		}
	}

	let overlapped_point_count = grid.iter().filter(|(_k, v)| v > &&1).count();
	println!("Part {}: {}", if part2 { "2" } else { "1" }, overlapped_point_count);
}


pub fn solve(inputs: Vec<String>) {
	// Tranform input lines into a vector of vectors of ints
	let inputs = inputs.iter().map(|input|
		input.split(" -> ").map(|part| part.split(",").map(|s| s.parse().unwrap()).collect_vec()).collect_vec()
	).collect_vec();

	// Now Map inner Vec<i64> to a Point struct
	let inputs = inputs.iter().map(|input| input.iter().map(|pt| Point { x: pt[0], y: pt[1]}).collect_vec()).collect_vec();

	sub_solve(&inputs, false /*part1*/);
	sub_solve(&inputs, true /*part1*/);
}