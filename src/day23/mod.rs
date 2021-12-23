use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, BinaryHeap};
use std::hash::{Hash, Hasher};

fn get_room(amphi: char) -> usize {
	((amphi as u32) - ('A' as u32)) as usize
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct State {
	rooms: [[char; 4]; 4],
	hallway: [char; 11],
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct BfsNode {
	energy: usize,
	state: State,
	previous_hash: u64,
}

impl Ord for BfsNode {
	fn cmp(&self, other: &Self) -> Ordering {
		self.energy.cmp(&other.energy).reverse()
	}
}

impl PartialOrd for BfsNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn get_hallway_slot_for_room(room: usize) -> usize {
	2 + room * 2
}

impl State {
	fn is_done(&self, room_size: usize) -> bool {
		self.rooms[0][0..room_size].iter().all(|&ch| ch == 'A') &&
		self.rooms[1][0..room_size].iter().all(|&ch| ch == 'B') &&
		self.rooms[2][0..room_size].iter().all(|&ch| ch == 'C') &&
		self.rooms[3][0..room_size].iter().all(|&ch| ch == 'D')
	}

	fn get_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish()
	}

	fn can_move_room_to_room(&self, src_room: usize, dst_room: usize) -> bool {
		if dst_room < src_room {
			self.can_move_room_to_room(dst_room, src_room)
		} else {
			let hw1 = get_hallway_slot_for_room(src_room);
			let hw2 = get_hallway_slot_for_room(dst_room);
			for i in hw1..=hw2 {
				if self.hallway[i] != ' ' {
					return false;
				}
			}
			return true;
		}
	}

	fn can_move_hallway_to_room(&self, src_hallway: usize, dst_room: usize) -> bool {
		let dst_hallway = get_hallway_slot_for_room(dst_room);

		let range = if dst_hallway < src_hallway { dst_hallway..=src_hallway-1 } else { src_hallway+1..=dst_hallway };
		for i in range {
			if self.hallway[i] != ' ' {
				return false;
			}
		}
		return true;
	}

	fn can_move_room_to_hallway(&self, src_room: usize, dst_hallway: usize) -> bool {
		let src_hallway = get_hallway_slot_for_room(src_room);

		let range = if dst_hallway < src_hallway { dst_hallway..=src_hallway } else { src_hallway..=dst_hallway };
		for i in range {
			if self.hallway[i] != ' ' {
				return false;
			}
		}
		return true;
	}
}

fn abs_diff(a: usize, b: usize) -> usize {
	if a > b {
		a - b
	} else {
		b - a
	}
}

fn dist_room_to_room(src_room: usize, src_slot: usize, dst_room: usize, dst_slot: usize) -> usize {
	let room_diff = abs_diff(src_room, dst_room);

	let slot_diff1 = src_slot + 1;
	let slot_diff2 = dst_slot + 1;

	slot_diff1 + slot_diff2 + room_diff * 2
}

fn energy_mul(amphi: char) -> usize {
	match amphi {
		'A' => 1,
		'B' => 10,
		'C' => 100,
		'D' => 1000,
		_ => unreachable!(),
	}
}

fn sub_solve(initial_rooms: &[[char; 4]; 4], room_size: usize) -> usize {
	let initial_state = State {
		hallway: [' '; 11],
		rooms: initial_rooms.clone(),
	};

	let mut heap = BinaryHeap::new();
	let mut seen = HashSet::new();

	heap.push(BfsNode{energy: 0, state: initial_state, previous_hash: 0});
	while !heap.is_empty() {
		let node = heap.pop().unwrap();

		if !seen.insert(node.state.clone()) {
			continue;
		}

		// println!("{}: {:?}\n", node.state.get_hash(), node);

		if node.state.is_done(room_size) {
			return node.energy;
		}

		// Move each room person out of room into (target room and every available hallway slot)
		for room in 0..4 {
			for slot in 0..room_size {
				let ch = node.state.rooms[room][slot];

				if ch == ' ' {
					continue;
				}

				let dest_room = get_room(ch);
				
				// Skip if we are in the right spot (and not blocking someone else from getting out), or if we're blocked from getting out ourselves
				if (room == dest_room && node.state.rooms[room][slot..room_size].iter().all(|&r| r == ch))
					 || node.state.rooms[room][0..slot].iter().any(|&r| r != ' ') {
					continue;
				}

				// Try destination room, each slot
				for dest_slot in (0..room_size).rev() {
					if node.state.rooms[dest_room][dest_slot] == ' ' {
						if node.state.can_move_room_to_room(room, dest_room) {
							let dist = dist_room_to_room(room, slot, dest_room, dest_slot);
	
							let mut new_state = node.state.clone();
							new_state.rooms[dest_room][dest_slot] = ch;
							new_state.rooms[room][slot] = ' ';
	
							heap.push(BfsNode{energy: node.energy + dist * energy_mul(ch), state: new_state, previous_hash: node.state.get_hash()});
						}
					} else if node.state.rooms[dest_room][dest_slot] != ch {
						// Found a foreign entity, bail
						break;
					}
				}

				// Now try each hallway slot
				for &hall in &[0, 1, 3, 5, 7, 9, 10] {
					if node.state.hallway[hall] == ' ' && node.state.can_move_room_to_hallway(room, hall) {
						let dist = abs_diff(get_hallway_slot_for_room(room), hall) + (slot + 1);

						let mut new_state = node.state.clone();
						new_state.hallway[hall] = ch;
						new_state.rooms[room][slot] = ' ';

						heap.push(BfsNode{energy: node.energy + dist * energy_mul(ch), state: new_state, previous_hash: node.state.get_hash()});
					}
				}
			}
		}

		// Now try to move any hallway person to their room
		for &hall in &[0, 1, 3, 5, 7, 9, 10] {
			let ch = node.state.hallway[hall];
			if ch != ' ' {
				let dest_room = get_room(ch);

				if node.state.can_move_hallway_to_room(hall, dest_room) {
					// Try destination room, each slot
					for dest_slot in (0..room_size).rev() {
						if node.state.rooms[dest_room][dest_slot] == ' ' {
							let dist = abs_diff(get_hallway_slot_for_room(dest_room), hall) + dest_slot + 1;

							let mut new_state = node.state.clone();
							new_state.rooms[dest_room][dest_slot] = ch;
							new_state.hallway[hall] = ' ';

							heap.push(BfsNode{energy: node.energy + dist * energy_mul(ch), state: new_state, previous_hash: node.state.get_hash()});
						} else if node.state.rooms[dest_room][dest_slot] != ch {
							// Found a foreign entity, bail
							break;
						}
					}
				}
			}
		}
	}

	unreachable!("Failed to find solution");
}


pub fn solve(inputs: Vec<String>) {
	let grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut initial_rooms = [[' '; 4]; 4];
	for r in 0..2 {
		for c in 0..4 {
			initial_rooms[c][r] = grid[r + 2][3 + c * 2];
		}
	}

	let initial_rooms_part2 = 
		[[initial_rooms[0][0], 'D', 'D', initial_rooms[0][1]],
		 [initial_rooms[1][0], 'C', 'B', initial_rooms[1][1]],
		 [initial_rooms[2][0], 'B', 'A', initial_rooms[2][1]],
		 [initial_rooms[3][0], 'A', 'C', initial_rooms[3][1]]];
	
	println!("Part 1: {}", sub_solve(&initial_rooms, 2 /*room size*/));

	println!("Part 2: {}", sub_solve(&initial_rooms_part2, 4 /*room size*/));
}