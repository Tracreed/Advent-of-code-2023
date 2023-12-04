use std::collections::BTreeMap;

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

	for (_, card) in cards {
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
	let mut map = BTreeMap::new();
	let mut sum = 0;

	for (_, card) in cards.iter() {
		*map.entry(card.card_number).or_insert(0) += 1;
	}

	for (id, card) in cards.iter() {
		let count = *map.entry(*id).or_insert(0);
		sum += count;
		if count > 0 {
			for next_id in (id + 1)..=(id + card.matching_card_count()) {
				*map.entry(next_id).or_insert(0) += count;
			}
		}
	}

	Some(sum)
}

fn parse_cards(input: &str) -> BTreeMap<u32, Card> {
	let mut cards = BTreeMap::new();

	for card in input.lines() {
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
		let mut card = card.split(": ").nth(1).unwrap().split(" | ");

		for number in card.next().unwrap().split_ascii_whitespace() {
			winning_numbers.push(number.parse::<u32>().unwrap());
		}

		for number in card.next().unwrap().split_ascii_whitespace() {
			numbers.push(number.parse::<u32>().unwrap());
		}

		cards.insert(
			card_number,
			Card {
				card_number,
				winning_numbers,
				numbers,
			},
		);
	}

	cards
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
