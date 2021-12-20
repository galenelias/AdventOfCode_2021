use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone)]
struct Image {
	pixels: HashSet<(i64, i64)>,
	min_r: i64,
	max_r: i64,
	min_c: i64,
	max_c: i64,
	void_pixel_on: bool,
}

impl Image {
	fn new(pixels: HashSet<(i64, i64)>, void_pixel_on: bool) -> Image {
		let min_r = pixels.iter().map(|c| c.0).min().unwrap();
		let max_r = pixels.iter().map(|c| c.0).max().unwrap();
		let min_c = pixels.iter().map(|c| c.1).min().unwrap();
		let max_c = pixels.iter().map(|c| c.1).max().unwrap();

		Image { pixels, min_r, max_r, min_c, max_c, void_pixel_on }
	}

	fn get(&self, r: i64, c: i64) -> usize {
		if r < self.min_r || r > self.max_r || c < self.min_c || c > self.max_c {
			self.void_pixel_on as usize
		} else {
			self.pixels.contains(&(r, c)) as usize
		}
	}
}

fn collect_bits(image: &Image, r: i64, c: i64) -> usize {
	image.get(r - 1, c - 1) << 8 |
	image.get(r - 1, c    ) << 7 |
	image.get(r - 1, c + 1) << 6 |
	image.get(r    , c - 1) << 5 |
	image.get(r    , c    )<< 4 |
	image.get(r    , c + 1) << 3 |
	image.get(r + 1, c - 1) << 2 |
	image.get(r + 1, c    ) << 1 |
	image.get(r + 1, c + 1) << 0
}

fn enhance(mut image: Image, key: &[bool], n: usize) -> Image {
	for _round in 0..n {
		let mut new_pixels: HashSet<(i64, i64)> = HashSet::new();
		for r in image.min_r-1..=image.max_r+1 {
			for c in image.min_c-1..=image.max_c+1 {
				let input_val = collect_bits(&image, r, c);

				if key[input_val] {
					new_pixels.insert((r, c));
				}
			}
		}

		let new_void_value = if image.void_pixel_on { key[511] } else { key[0] };
		image = Image::new(new_pixels, new_void_value);
	}
	return image;
}

pub fn solve(inputs: Vec<String>) {
	let key: Vec<bool> = inputs[0].chars().map(|ch| match ch { '#' => true, _ => false }).collect_vec();
	let inputs: Vec<Vec<char>> = inputs.into_iter().skip(2).map(|line| line.chars().collect_vec()).collect();

	let mut initial_pixels: HashSet<(i64, i64)> = HashSet::new();
	for r in 0..inputs.len() {
		for c in 0..inputs[r].len() {
			if inputs[r][c] == '#' {
				initial_pixels.insert((r as i64, c as i64));
			}
		}
	}

	let original_image = Image::new(initial_pixels, false /*void_pixel_on*/);

	let part1_image = enhance(original_image.clone(), &key, 2);
	println!("Part 1: {}", part1_image.pixels.len());

	let part2_image = enhance(original_image.clone(), &key, 50);
	println!("Part 2: {}", part2_image.pixels.len());
}