use std::collections::VecDeque;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
	card_number: u32,
	winning_numbers: Vec<u32>,
	numbers: Vec<u32>,
}

impl Card {
	fn matching_card_count(&self) -> u32 {
		self.numbers
			.iter()
			.filter(|&x| self.winning_numbers.contains(x))
			.count() as u32
	}
}

pub fn part_one(input: &str) -> Option<u32> {
	let mut sum = 0;
	let cards = parse_cards(input);

	for card in cards {
		let mut current_worth = 0.5;

		for _ in 0..card.matching_card_count() {
			current_worth *= 2.0;
		}

		sum += current_worth as u32;
	}

	Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
	let cards = parse_cards(input);
	let mut process_cards = VecDeque::from(cards.clone());
	let mut sum = 0;

	while let Some(card) = process_cards.pop_front() {
		let winning_count = card.matching_card_count();
		let start = card.card_number;
		let end = start + winning_count;

		let winning_card = cards.get(start as usize..end as usize).unwrap();

		for c in winning_card.iter().rev() {
			process_cards.push_front(c.clone());
		}
		sum += 1;
	}

	Some(sum)
}

fn parse_cards(input: &str) -> Vec<Card> {
	input
		.lines()
		.map(|card| {
			let mut winning_numbers = Vec::new();
			let mut numbers = Vec::new();

			let card_number = card
				.split(": ")
				.next()
				.unwrap()
				.split_ascii_whitespace()
				.nth(1)
				.unwrap()
				.parse::<u32>()
				.unwrap();
			let mut card_parts = card.split(": ").nth(1).unwrap().split(" | ");

			winning_numbers.extend(
				card_parts
					.next()
					.unwrap()
					.split(' ')
					.map(str::trim)
					.filter_map(|x| x.parse::<u32>().ok()),
			);
			numbers.extend(
				card_parts
					.next()
					.unwrap()
					.split(' ')
					.map(str::trim)
					.filter_map(|x| x.parse::<u32>().ok()),
			);

			Card {
				card_number,
				winning_numbers,
				numbers,
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(13));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(30));
	}
}
