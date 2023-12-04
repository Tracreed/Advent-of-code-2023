use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
	let mut sum = 0;
	let schematic = get_schematic(input);

	let mut current_number = String::new();
	let mut adjacent = false;
	for (current_line, line) in schematic.iter().enumerate() {
		for (current_character, c) in line.iter().enumerate() {
			if c.parse::<u32>().is_ok() {
				current_number = current_number + c;
				if adjacent {
					continue;
				}
				let adjacent_characters =
					get_adjacent(&schematic, current_line, current_character);

				adjacent = adjacent_characters
					.iter()
					.any(|character| character.parse::<u32>().is_err() && character != ".");
			} else if !current_number.is_empty() {
				if adjacent {
					sum += current_number.parse::<u32>().unwrap();
					adjacent = false;
				}
				current_number = String::new();
			}
		}
		if !current_number.is_empty() {
			if adjacent {
				sum += current_number.parse::<u32>().unwrap();
				adjacent = false;
			}
			current_number = String::new();
		}
	}
	Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut sum = 0;
	let mut schematic = get_schematic(input);

	let mut id = 0;
	for (_, line) in schematic.iter_mut().enumerate() {
		for (_, c) in line.iter_mut().enumerate() {
			if c == "*" {
				id += 1;
				let new_id = format!("{}{}", c, id);
				*c = new_id;
			}
		}
	}

	let mut current_number = String::new();
	let mut adjacent_id = 0;
	let mut ids = HashMap::new();
	for (current_line, line) in schematic.iter().enumerate() {
		for (current_character, c) in line.iter().enumerate() {
			if c.parse::<u32>().is_ok() {
				current_number = current_number + c;
				let adjacent_characters = get_adjacent(&schematic, current_line, current_character);

				if adjacent_id == 0 {
					adjacent_id = adjacent_characters
						.iter()
						.find(|character| character.starts_with('*'))
						.map(|character| character.trim_start_matches('*').parse::<u32>().unwrap())
						.unwrap_or(0);
				}
			} else if !current_number.is_empty() {
				if adjacent_id != 0 {
					ids.entry(adjacent_id)
						.or_insert(Vec::new())
						.push(current_number.parse::<u32>().unwrap());
					adjacent_id = 0;
				}
				current_number = String::new();
			}
		}
		if !current_number.is_empty() {
			if adjacent_id != 0 {
				ids.entry(adjacent_id)
					.or_insert(Vec::new())
					.push(current_number.parse::<u32>().unwrap());
				adjacent_id = 0;
			}
			current_number = String::new();
		}
	}
	for (_, numbers) in ids.iter() {
		if numbers.len() > 1 {
			let mut product = 1;
			for number in numbers.iter() {
				product *= number;
			}
			sum += product;
		}
	}
	Some(sum)
}

fn get_adjacent(
	schematic: &[Vec<String>],
	current_line: usize,
	current_character: usize,
) -> Vec<String> {
	let mut neighbors = Vec::new();
	for (line_index, line) in schematic.iter().enumerate() {
		for (character_index, c) in line.iter().enumerate() {
			if line_index == current_line && character_index == current_character {
				continue;
			}
			if line_index >= current_line - 1
				&& line_index <= current_line + 1
				&& character_index >= current_character - 1
				&& character_index <= current_character + 1
			{
				neighbors.push(c.to_owned());
			}
		}
	}
	neighbors
}

fn get_schematic(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(4361));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(467835));
	}
}
