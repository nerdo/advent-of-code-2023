//! Island Island Almanac
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day6_get_lowest_location_number_that_corresponds_to_any_initial_seed_number_returns_the_correct_answer(
    ) {
        let almanac_text = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ";

        let almanac = Almanac::from(almanac_text);

        let result =
            almanac.get_lowest_location_number_that_corresponds_to_any_initial_seed_number();

        assert_eq!(result, 35);
    }
}

/// The Island Island Almanac
pub struct Almanac {}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl Almanac {
    fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(&self) -> u64 {
        todo!()
    }
}

pub fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(
    almanac_text: &str,
) -> u64 {
    let almanac = Almanac::from(almanac_text);
    almanac.get_lowest_location_number_that_corresponds_to_any_initial_seed_number()
}
