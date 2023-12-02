advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
	let mut sum: u32 = 0;
	let (red, green, blue) = (12, 13, 14);

	for line in input.lines().enumerate() {
		let (reds, greens, blues) = get_numbered_colors(line.1);

		if reds.iter().any(|&x| x > red)
			|| greens.iter().any(|&x| x > green)
			|| blues.iter().any(|&x| x > blue)
		{
			continue;
		}

		sum += (line.0 + 1) as u32;
	}

	Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut sum: u32 = 0;
	for line in input.lines() {
		let (reds, greens, blues) = get_numbered_colors(line);

		let (red_min, greens_min, blues_min) = (
			reds.iter().max()?,
			greens.iter().max()?,
			blues.iter().max()?,
		);

		sum += red_min * greens_min * blues_min;
	}

	Some(sum)
}

fn get_colors(line: &str) -> Vec<&str> {
	let colors: Vec<&str> = line
		.split_whitespace()
		.map(|s| s.trim_end_matches(|c| c == ',' || c == ';'))
		.filter(|s| *s == "red" || *s == "green" || *s == "blue")
		.collect();

	colors
}

fn get_numbers(line: &str) -> Vec<u32> {
	let numbers: Vec<u32> = line
		.split_whitespace()
		.map(|s| s.parse::<u32>())
		.filter_map(Result::ok)
		.collect();

	numbers
}

fn get_numbered_colors(line: &str) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
	let (mut reds, mut greens, mut blues) = (Vec::new(), Vec::new(), Vec::new());
	let colors = get_colors(line);
	let numbers = get_numbers(line);

	for (i, color) in colors.iter().enumerate() {
		match *color {
			"red" => reds.push(numbers[i]),
			"green" => greens.push(numbers[i]),
			"blue" => blues.push(numbers[i]),
			_ => (),
		}
	}

	(reds, greens, blues)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(8));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(2286));
	}
}
