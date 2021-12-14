use itertools::Itertools;
use std::collections::HashMap;

fn is_small(node_name: &str) -> bool {
    node_name.chars().all(|ch| ch.is_lowercase())
}

fn sub_solve<'a>(
    node_connections: &HashMap<&'a str, Vec<&'a str>>,
    node_str: &'a str,
    visited: &mut Vec<&'a str>,
    mut allow_double_dip: bool,
) -> usize {
    if node_str == "end" {
        return 1;
    }

    if is_small(node_str) && visited.contains(&node_str) {
        if !allow_double_dip || node_str == "start" {
            return 0;
        } else {
            allow_double_dip = false;
        }
    }

    visited.push(node_str);
    let sub_sum = node_connections
        .get(node_str).unwrap()
        .iter()
        .map(|connection| sub_solve(node_connections, connection, visited, allow_double_dip))
        .sum::<usize>();
    visited.pop();

    return sub_sum;
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

    println!( "Part 1: {}", sub_solve( &nodes, "start", &mut Vec::new(), false /*allow_double_dip*/));
    println!( "Part 2: {}", sub_solve( &nodes, "start", &mut Vec::new(), true /*allow_double_dip*/));
}
