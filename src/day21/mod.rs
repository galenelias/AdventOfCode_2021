use std::collections::HashMap;

struct Die {
	face: usize,
	roll_count: usize,
}

impl Die {
	fn roll(&mut self) -> usize {
		let result = self.face;
		self.face += 1;
		if self.face > 100 {
			self.face = 1;
		}
		self.roll_count += 1;
		result
	}

	fn roll3(&mut self) -> usize {
		self.roll() + self.roll() + self.roll()
	}
}

const PART1_MAX_SCORE: usize = 1000;

type MemoHash = HashMap<(bool, (usize, usize), (usize, usize)), (usize, usize)>;

fn quantum(p1turn: bool, positions: (usize, usize), scores: (usize, usize), memo: &mut MemoHash) -> (usize, usize) {
	if scores.0 >= 21 || scores.1 >= 21 {
		if scores.0 > scores.1 {
			(1, 0)
		} else {
			(0, 1)
		}
	} else {
		if let Some(memo) = memo.get(&(p1turn, positions, scores)) {
			return *memo;
		}

		let mut result = (0usize, 0usize);
		for die1 in 1..=3 {
			for die2 in 1..=3 {
				for die3 in 1..=3 {
					let dice = die1 + die2 + die3;
					if p1turn {
						let new_pos = ((positions.0 + dice) % 10, positions.1);
						let sub = quantum(false, new_pos, (scores.0 + new_pos.0 + 1, scores.1), memo);
						result = (result.0 + sub.0, result.1 + sub.1);	
					} else {
						let new_pos = (positions.0, (positions.1 + dice) % 10);
						let sub = quantum(true, new_pos, (scores.0, scores.1 + new_pos.1 + 1), memo);
						result = (result.0 + sub.0, result.1 + sub.1);
					}
				}
			}
		}
		memo.insert((p1turn, positions, scores), result);
		result
	}
}

pub fn solve(inputs: Vec<String>) {
	let input_positions: Vec<_> = inputs.iter().map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1).collect();

	let mut positions = input_positions.clone();
	let mut scores = [0,0];
	let mut die = Die { face: 1, roll_count: 0 };

	while scores.iter().all(|s| s < &PART1_MAX_SCORE) {
		for p in 0..2 {
			let roll3 = die.roll3();
			positions[p] += roll3;
			positions[p] = positions[p] % 10;
			scores[p] += positions[p] + 1;

			if scores[p] >= PART1_MAX_SCORE {
				break;
			}
		}
	}

	let losing = scores.iter().min().unwrap();
	println!("Part 1: {} * {} = {}", losing, die.roll_count, losing * die.roll_count);

	let mut memo_hash = HashMap::new();
	let p2 = quantum(true, (input_positions[0], input_positions[1]), (0, 0), &mut memo_hash);
	println!("Part 2: {}", std::cmp::max(p2.0, p2.1));
}