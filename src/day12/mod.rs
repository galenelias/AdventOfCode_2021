use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn is_small(node_name: &str) -> bool {
	node_name.chars().all(|ch| ch.is_lowercase())
}

fn sub_solve<'a>(node_connections: &HashMap<&'a str, Vec<&'a str>>, allow_double_dip: bool) -> usize {
	let mut path_count = 0;

	let mut q: Vec<(&str, HashSet<&str>, bool)> = Vec::new();
	q.push(("start", HashSet::new(), false));

	while !q.is_empty() {
		let (node_str, mut visited, mut did_double_dip) = q.pop().unwrap();

		if is_small(node_str) && visited.contains(node_str) {
			if !allow_double_dip || did_double_dip || node_str == "start" {
				continue;
			} else {
				did_double_dip = true;
			}
		}

		visited.insert(node_str);
		for connection in node_connections.get(node_str).unwrap() {
			if *connection == "end" {
				path_count += 1;
			} else {
				q.push((connection, visited.clone(), did_double_dip));
			}
		}
	}

	return path_count;
}

pub fn solve(inputs: Vec<String>) {
	let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

	for input in &inputs {
		let parts = input.split('-').collect_vec();

		let p1 = nodes.entry(parts[0]).or_insert(Vec::new());
		(*p1).push(parts[1]);

		let p2 = nodes.entry(parts[1]).or_insert(Vec::new());
		(*p2).push(parts[0]);
	}

	println!("Part 1: {}", sub_solve(&nodes, false /*allow_double_dip*/));
	println!("Part 2: {}", sub_solve(&nodes, true /*allow_double_dip*/));
}
