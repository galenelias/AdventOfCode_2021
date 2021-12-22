use regex::Regex;
use std::collections::HashSet;
use itertools::Itertools;

struct Rect {
	on: bool,
	x1: i32,
	x2: i32,
	y1: i32,
	y2: i32,
	z1: i32,
	z2: i32,
}

pub fn solve(inputs: Vec<String>) {
	let mut grid: HashSet<(i32,i32,i32)> = HashSet::new();

	let re_line = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

	let rects: Vec<_> = inputs.iter().map(|input| {
		let caps = re_line.captures(&input).unwrap();
		Rect {
			on: &caps[1] == "on",
			x1: caps[2].parse::<i32>().unwrap(),
			x2: caps[3].parse::<i32>().unwrap()+1,
			y1: caps[4].parse::<i32>().unwrap(),
			y2: caps[5].parse::<i32>().unwrap()+1,
			z1: caps[6].parse::<i32>().unwrap(),
			z2: caps[7].parse::<i32>().unwrap()+1,
		}
	}).collect();

	for r in &rects {
		for z in std::cmp::max(r.z1,-50)..std::cmp::min(r.z2,51) {
			for y in std::cmp::max(r.y1,-50)..std::cmp::min(r.y2,51) {
				for x in std::cmp::max(r.x1,-50)..std::cmp::min(r.x2,51) {
					if r.on {
						grid.insert((x, y, z));
					} else {
						grid.remove(&(x, y, z));
					}
				}
			}
		}
	}

	println!("Part 1: {}", grid.len());

	let mut zset: HashSet<i32> = HashSet::new();
	let mut xset: HashSet<i32> = HashSet::new();
	let mut yset: HashSet<i32> = HashSet::new();
	for r in &rects {
		zset.insert(r.z1);
		zset.insert(r.z2);
		yset.insert(r.y1);
		yset.insert(r.y2);
		xset.insert(r.x1);
		xset.insert(r.x2);
	}

	let zs: Vec<_> = zset.into_iter().sorted().collect();
	let ys: Vec<_> = yset.into_iter().sorted().collect();
	let xs: Vec<_> = xset.into_iter().sorted().collect();

	let mut part2: usize = 0;
	for zi in 0..zs.len()-1 {
		for yi in 0..ys.len()-1 {
			for xi in 0..xs.len()-1 {
				let mut is_on = false;
				for r in rects.iter().rev() {
					if zs[zi] >= r.z1 && zs[zi] < r.z2
						&& ys[yi] >= r.y1 && ys[yi] < r.y2
						&& xs[xi] >= r.x1 && xs[xi] < r.x2 {
							is_on = r.on;
							break;
						}
				}
				if is_on {
					part2 += (zs[zi+1] - zs[zi]) as usize * (ys[yi+1] - ys[yi]) as usize * (xs[xi+1] - xs[xi]) as usize;
				}
			}
		}
	}

	println!("Part 2: {}", part2);
}
