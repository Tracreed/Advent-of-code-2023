advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
	let mut sum = 0;

	for line in input.lines() {
		let mut numbers = Vec::new();
		line.chars()
			.filter(|c| c.is_ascii_digit())
			.for_each(|c| numbers.push(c.to_digit(10).unwrap()));
		sum += format!(
			"{}{}",
			numbers.first().unwrap_or(&0),
			numbers.last().unwrap_or(&0)
		)
		.parse::<u32>()
		.unwrap();
	}

	Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut sum = 0;

	for line in input.lines() {
		let mut numbers = Vec::new();
		let line = line
			.replace("one", "o1e")
			.replace("two", "t2o")
			.replace("three", "t3e")
			.replace("four", "f4r")
			.replace("five", "f5e")
			.replace("six", "s6x")
			.replace("seven", "s7n")
			.replace("eight", "e8t")
			.replace("nine", "n9e");
		line.chars()
			.filter(|c| c.is_ascii_digit())
			.for_each(|c| numbers.push(c.to_digit(10).unwrap()));
		sum += format!(
			"{}{}",
			numbers.first().unwrap_or(&0),
			numbers.last().unwrap_or(&0)
		)
		.parse::<u32>()
		.unwrap();
	}

	Some(sum)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
		assert_eq!(result, Some(142));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
		assert_eq!(result, Some(281));
	}
}
