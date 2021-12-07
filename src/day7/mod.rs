pub fn solve(inputs: Vec<String>) {
    let crabs: Vec<i64> = inputs[0].split(",").map(|i| i.parse().unwrap()).collect();

    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();

    let part1: i64 = (min_crab..=max_crab)
        .map(|dest| crabs.iter().map(|input| (input - dest).abs()).sum())
        .min()
        .unwrap();

    let gauss_sum = |num| (num * (num + 1)) / 2;
    let part2: i64 = (min_crab..=max_crab)
        .map(|dest| {
            crabs
                .iter()
                .map(|input| gauss_sum((input - dest).abs()))
                .sum()
        })
        .min()
        .unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
