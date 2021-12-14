use itertools::Itertools;
use std::collections::HashMap;

fn compute_delta(pair_freqs: &HashMap<(char, char), usize>, last_char: char) -> usize {
	// Compute frequency of characters by looking at first character in each pair
	let mut char_freqs: HashMap<char, usize> = HashMap::new();
	for (k, v) in pair_freqs {
		*char_freqs.entry(k.0).or_default() += v;
	}

	// Add in last element of the polymer, as it's not captured in our pair based computation, and doesn't change
	(*char_freqs.entry(last_char).or_default()) += 1;

	char_freqs.values().max().unwrap() - char_freqs.values().min().unwrap()	
}

pub fn solve(inputs: Vec<String>) {
	let polymer = inputs[0].chars().collect_vec();
	let last_char = *polymer.last().unwrap();

	let mut rules = HashMap::new();
	for rule in &inputs[2..] {
		let (lhs, rhs) = rule.split_once(" -> ").unwrap();
		let lhs = lhs.chars().collect_vec();
		rules.insert((lhs[0], lhs[1]), rhs.chars().next().unwrap());
	}

	let mut pair_freqs: HashMap<(char, char), usize> = HashMap::new();
	for (a, b) in polymer.iter().tuple_windows() {
		(*pair_freqs.entry((*a, *b)).or_default()) += 1;
	}

	for i in 0..40 {
		let mut new_pair_freqs: HashMap<(char, char), usize> = HashMap::new();

		for (&(a, b), v) in &pair_freqs {
			let to_insert = *rules.get(&(a, b)).unwrap();
			(*new_pair_freqs.entry((a, to_insert)).or_default()) += v;
			(*new_pair_freqs.entry((to_insert, b)).or_default()) += v;
		}

		pair_freqs = new_pair_freqs;

		if i == 9 {
			println!("Part 1: {}", compute_delta(&pair_freqs, last_char));
		}
	}

	println!("Part 2: {}", compute_delta(&pair_freqs, last_char));
}