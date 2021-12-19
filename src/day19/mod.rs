
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

type Point = [i64; 3];
type Points = Vec<Point>;

const AXIS_SWAP_MAX: usize = 6;
const SIGN_TOGGLE_MAX: usize = 8;

fn permute(points: &Points, axis_swap: usize, sign_toggle: usize) -> Points {
	let axis_swizzles = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

	points.iter().map(|pt| {
		let mut res = [pt[axis_swizzles[axis_swap][0]], pt[axis_swizzles[axis_swap][1]], pt[axis_swizzles[axis_swap][2]]];
		for axis in 0..3 {
			if (sign_toggle & (1 << axis)) != 0 {
				res[axis] *= -1;
			}
		}
		res
	}).collect()
}

// If we can find 12 common points, merge the incoming points and return the scanner offset (or None if no common points were found)
fn _merge_orig(anchor: &mut HashSet<Point>, incoming: &Points) -> Option<Point> {
	for pt1 in anchor.iter() {
		for pt2 in incoming {
			let diff = [pt1[0] - pt2[0], pt1[1] - pt2[1], pt1[2] - pt2[2]];

			let adjusted_pts: HashSet<_> = incoming.iter().map(|pt| [pt[0] + diff[0], pt[1] + diff[1], pt[2] + diff[2]]).collect();

			let common_pts = anchor.intersection(&adjusted_pts).count();
			if common_pts >= 12 {
				anchor.extend(adjusted_pts.into_iter());
				return Some(diff);
			}
		}
	}

	None
}

// Andy's algorithm for detecting overlaps via hashing the deltas to see if enough pairs share the same delta.
fn merge_fast(anchor: &mut HashSet<Point>, incoming: &Points) -> Option<Point> {
	let mut deltas: HashMap<Point, usize> = HashMap::new();

	for pt1 in anchor.iter() {
		for pt2 in incoming {
			let diff = [pt1[0] - pt2[0], pt1[1] - pt2[1], pt1[2] - pt2[2]];

			let entry = deltas.entry(diff).or_default();
			*entry += 1;
			if *entry >= 12 {
				let adjusted_pts: HashSet<_> = incoming.iter().map(|pt| [pt[0] + diff[0], pt[1] + diff[1], pt[2] + diff[2]]).collect();
				anchor.extend(adjusted_pts.into_iter());
				return Some(diff);
			}
		}
	}

	None
}

pub fn solve(inputs: Vec<String>) {
	let mut scanners: Vec<Points> = Vec::new();
	let mut scanner: Points = Vec::new();

	for line in inputs {
		if line.starts_with("---") {
			continue;
		} else if line.len() == 0 {
			scanners.push(scanner);
			scanner = Vec::new();
		} else {
			let parts = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect_vec();
			scanner.push([parts[0], parts[1], parts[2]]);
		}
	}

	if !scanner.is_empty() {
		scanners.push(scanner);
	}

	let mut anchor: HashSet<_> = scanners.pop().unwrap().into_iter().collect();
	let mut scanner_positions = Vec::new();

	while !scanners.is_empty() {
		'outer: for axis_swap in 0..AXIS_SWAP_MAX {
			for sign_toggle in 0..SIGN_TOGGLE_MAX {
				for i in 0..scanners.len() {
					let points = permute(&scanners[i], axis_swap, sign_toggle);

					if let Some(scanner_offset) = merge_fast(&mut anchor, &points) {
						scanners.remove(i);
						scanner_positions.push(scanner_offset);
						break 'outer;
					}
				}
			}
		}
	}

	println!("Part 1: {}", anchor.len());

	let part2 = scanner_positions.iter()
		.map(|pt1| {
			scanner_positions.iter().map(|pt2| (pt2[0]-pt1[0]).abs() + (pt2[1]-pt1[1]).abs() + (pt2[2]-pt1[2]).abs()).max().unwrap()
		}).max().unwrap();

	println!("Part 2: {}", part2);
}