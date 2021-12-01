use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<u32>().unwrap()).collect_vec();

	let part1 = inputs.windows(2).filter(|pair| pair[1] > pair[0]).count();
	println!("Part 1: {}", part1);

	let sliding_sums: Vec<u32> = inputs.windows(3).map(|tuple| tuple.iter().sum()).collect();
	let part2 = sliding_sums.windows(2).filter(|pair| pair[1] > pair[0]).count();
	println!("Part 2: {}", part2);
}