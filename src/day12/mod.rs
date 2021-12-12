use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Node {
	is_big: bool,
	connections: Vec<String>,
}

impl Node {
	pub fn new(node_name: &str) -> Self {
		Node {
			is_big: node_name.chars().all(|ch| ch.is_uppercase()),
			connections: Vec::new(),
		}
	}
}

fn sub_solve(nodes: &HashMap<String, Node>, allow_double_dip: bool) -> usize {
	let mut path_count = 0;

	let mut q: VecDeque<(Vec<String>, bool)> = VecDeque::new();
	q.push_back((vec![String::from("start")], false));

	while !q.is_empty() {
		let (path, mut did_double_dip) = q.pop_front().unwrap();
		let node_str = path.last().unwrap();

		if node_str == "end" {
			path_count += 1;
			continue;
		}

		let node = nodes.get(node_str).unwrap();

		if !node.is_big && path.iter().filter(|&e| e == node_str).count() > 1 {
			if !allow_double_dip || did_double_dip || node_str == "start" {
				continue;
			} else {
				did_double_dip = true;
			}
		}

		for connection in &node.connections {
			let mut new_path = path.clone();
			new_path.push(connection.clone());
			q.push_back((new_path, did_double_dip));
		}
	}

	return path_count;
}

pub fn solve(inputs: Vec<String>) {
	let mut nodes: HashMap<String, Node> = HashMap::new();

	for input in inputs {
		let parts = input.split('-').collect_vec();

		{
			let p1 = nodes.entry(parts[0].to_owned()).or_insert(Node::new(parts[0]));
			(*p1).connections.push(parts[1].to_owned());
		}

		{
			let p2 = nodes.entry(parts[1].to_owned()).or_insert(Node::new(parts[1]));
			(*p2).connections.push(parts[0].to_owned());
		}
	}

	println!("Part 1: {}", sub_solve(&nodes, false /*allow_double_dip*/));
	println!("Part 2: {}", sub_solve(&nodes, true /*allow_double_dip*/));
}