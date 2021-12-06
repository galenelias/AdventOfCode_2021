use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let input: Vec<usize> = inputs[0].split(",").map(|i| i.parse().unwrap()).collect_vec();

	// Count of fish by age
	let mut age_buckets = vec![0; 11];
	for fish in input {
		age_buckets[fish] += 1;
	}

	for i in 0..256 {
		if i == 80 {
			println!("Part 1: {}", age_buckets.iter().sum::<usize>());
		}

		for age in 0..10 {
			if age == 0 {
				age_buckets[7] += age_buckets[0]; // move reproducing fish back to age 7
				age_buckets[9] = age_buckets[0]; // spawn new fish
			}
			age_buckets[age] = age_buckets[age+1];
		}

	}

	println!("Part 2: {}", age_buckets.iter().sum::<usize>());
}