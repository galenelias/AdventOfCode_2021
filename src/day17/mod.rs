use regex::Regex;

struct Rect {
	x1: i64,
	x2: i64,
	y1: i64,
	y2: i64,
}

fn can_hit_dx(mut dx: i64, target: &Rect) -> bool {
	let mut x = 0;

	while x <= target.x2 && dx > 0 {
		x += dx;
		dx -= dx.signum(); // drag

		if x >= target.x1 && x <= target.x2 {
			return true;
		}
	}

	return false;
}

fn trace(mut dx: i64, mut dy: i64, target: &Rect) -> (bool, i64) {
	let mut x = 0;
	let mut y = 0;
	let mut max_y = 0;

	while x <= target.x2 && (y >= target.y1 || dy >= 0) {
		x += dx;
		y += dy;

		max_y = std::cmp::max(y, max_y);

		dx -= dx.signum(); // drag
		dy -= 1; // gravity

		if x >= target.x1 && x <= target.x2 && y >= target.y1 && y <= target.y2 {
			return (true, max_y);
		}
	}

	return (false, max_y);
}

pub fn solve(inputs: Vec<String>) {
	let re_line = Regex::new(r"target area: x=([-\d]+)..([-\d]+), y=([-\d]+)..([-\d]+)").unwrap();
	let caps = re_line.captures(&inputs[0]).unwrap();
	let target = Rect{
		x1: caps[1].parse::<i64>().unwrap(),
		x2: caps[2].parse::<i64>().unwrap(),
		y1: caps[3].parse::<i64>().unwrap(),
		y2: caps[4].parse::<i64>().unwrap(),
	};

	let mut max_y = 0;
	let mut hits = 0;

	for dx in 1..=target.x2 {
		if !can_hit_dx(dx, &target) {
			continue;
		}

		for dy in (-100..100).rev() {
			let (hit, hit_max_y) = trace(dx, dy, &target);
			if hit {
				hits += 1;
				max_y = std::cmp::max(max_y, hit_max_y);
			}
		}
	}
	println!("Part 1: {}", max_y);
	println!("Part 2: {}", hits);
}