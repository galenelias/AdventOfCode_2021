use itertools::Itertools;
use std::collections::HashMap;

struct Point {
	x: i64,
	y: i64,
}

fn sub_solve(point_pairs: &Vec<Vec<Point>>, part2: bool) {
	let mut grid: HashMap<(i64, i64), usize> = HashMap::new();

	for points in point_pairs {
		if points[0].x == points[1].x { // vertical line
			let x = points[0].x;
			let lower = std::cmp::min(points[0].y, points[1].y);
			let upper = std::cmp::max(points[0].y, points[1].y);
			for y in lower..=upper {
				(*grid.entry((x, y)).or_default()) += 1;
			}
		} else if points[0].y == points[1].y { // horizontal line
			let y = points[0].y;
			let lower = std::cmp::min(points[0].x, points[1].x);
			let upper = std::cmp::max(points[0].x, points[1].x);
			for x in lower..=upper {
				(*grid.entry((x, y)).or_default()) += 1;
			}
		} else if part2 {
			let dx = (points[1].x - points[0].x) / (points[1].x - points[0].x).abs();
			let dy = (points[1].y - points[0].y) / (points[1].y - points[0].y).abs();

			let mut x = points[0].x;
			let mut y = points[0].y;

			loop {
				(*grid.entry((x, y)).or_default()) += 1;
				if x == points[1].x {
					break;
				}
				x += dx;
				y += dy;
			}
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