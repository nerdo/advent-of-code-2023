//! Scratch Cards.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day4_get_total_scratch_card_points_returns_the_correct_answer() {
        let input = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let result = get_total_scratch_card_points(input);

        assert_eq!(result, 13);
    }
}

/// Gets the total number of points from the scratch card data.
pub fn get_total_scratch_card_points(scratchcard_table: &str) -> u64 {
    let scratch_card_data = ScratchCardData::from(scratchcard_table);
    scratch_card_data.get_total_points()
}

struct ScratchCardData {
    card_data: Vec<CardData>,
}

impl ScratchCardData {
    fn get_total_points(&self) -> u64 {
        self.card_data.iter().map(|card| card.get_points()).sum()
    }
}

impl From<&str> for ScratchCardData {
    fn from(value: &str) -> Self {
        let card_data: Result<Vec<CardData>, _> = value
            .lines()
            .filter_map(|line| match CardData::try_from(line) {
                Err(ParseError::EmptyLine) => None,
                result => Some(result),
            })
            .collect();
        match card_data {
            Ok(card_data) => Self { card_data },
            Err(e) => panic!("{e:?}"),
        }
    }
}

struct CardData {
    id: u64,
    winning_numbers: HashSet<u64>,
    actual_numbers: Vec<u64>,
}

impl CardData {
    fn get_points(&self) -> u64 {
        let num_winning_numbers = self
            .actual_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32;
        match num_winning_numbers {
            0 => 0,
            _ => 2_u64.pow(num_winning_numbers - 1),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    EmptyLine,
    Header(String),
    Contents(String),
}

impl TryFrom<&str> for CardData {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let line = value.trim();
        if line.is_empty() {
            return Err(ParseError::EmptyLine);
        }

        let (header, contents) = line
            .split_once(':')
            .ok_or_else(|| ParseError::Header(line.to_owned()))?;

        let id = {
            let (_, id_text) = header
                .split_once(' ')
                .ok_or_else(|| ParseError::Header(line.to_owned()))?;
            id_text
                .trim()
                .parse::<u64>()
                .map_err(|e| ParseError::Header(e.to_string()))?
        };

        let (winning_numbers, actual_numbers) = {
            let (winning_part, actual_part) = contents
                .split_once('|')
                .ok_or_else(|| ParseError::Contents(contents.to_owned()))?;

            let winning = winning_part
                .split_whitespace()
                .map(|n| {
                    n.parse::<u64>()
                        .map_err(|e| ParseError::Contents(e.to_string()))
                })
                .collect::<Result<HashSet<_>, ParseError>>()?;
            let actual = actual_part
                .split_whitespace()
                .map(|n| {
                    n.parse::<u64>()
                        .map_err(|e| ParseError::Contents(e.to_string()))
                })
                .collect::<Result<Vec<_>, ParseError>>()?;

            (winning, actual)
        };

        Ok(Self {
            id,
            winning_numbers,
            actual_numbers,
        })
    }
}
