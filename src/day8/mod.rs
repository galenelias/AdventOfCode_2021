use itertools::Itertools;
use std::collections::HashMap;

fn translate_digits(input: &str, permutation: &Vec<char>) -> String {
    input
        .chars()
        .map(|ch| {
            let ordinal = (ch as u32) - ('a' as u32);
            permutation[ordinal as usize]
        })
        .sorted()
        .collect::<String>()
}

pub fn solve(inputs: Vec<String>) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut reference_keys = HashMap::new();
    reference_keys.insert("abcefg", 0);
    reference_keys.insert("cf", 1);
    reference_keys.insert("acdeg", 2);
    reference_keys.insert("acdfg", 3);
    reference_keys.insert("bcdf", 4);
    reference_keys.insert("abdfg", 5);
    reference_keys.insert("abdefg", 6);
    reference_keys.insert("acf", 7);
    reference_keys.insert("abcdefg", 8);
    reference_keys.insert("abcdfg", 9);

    for input in inputs {
        let parts = input.split(" | ").collect_vec();
        let unique_patterns = parts[0].split_whitespace().collect_vec();
        let output_values = parts[1].split_whitespace().collect_vec();

        for permutation in ['a', 'b', 'c', 'd', 'e', 'f', 'g']
            .iter()
            .cloned()
            .permutations(7)
        {
            let mut seen_digits = vec![0; 10];

            for unique in &unique_patterns {
                let translation = translate_digits(unique, &permutation);
                if let Some(digit) = reference_keys.get(&translation as &str) {
                    seen_digits[*digit] += 1;
                } else {
                    break;
                }
            }

            // We found a valid mapping if all digits were produced exactly once
            if seen_digits.iter().all(|&c| c == 1) {
                let output_nums = output_values
                    .iter()
                    .map(|output| {
                        let translation = translate_digits(output, &permutation);
                        return reference_keys.get(&translation as &str).unwrap();
                    })
                    .collect_vec();

                part1 += output_nums
                    .iter()
                    .map(|digit| match digit {
                        1 | 4 | 7 | 8 => 1,
                        _ => 0,
                    })
                    .sum::<u32>();

                part2 += output_nums[0] * 1000
                    + output_nums[1] * 100
                    + output_nums[2] * 10
                    + output_nums[3];

				break;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
