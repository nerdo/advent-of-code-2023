//! Engine for gondola lift.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_get_sum_of_all_engine_part_numbers_returns_correct_answer() {
        let engine_schematic = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ";

        let result = get_sum_of_all_engine_part_numbers(engine_schematic);

        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_get_sum_of_all_engine_gear_ratios_returns_the_correct_answer() {
        let engine_schematic = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        ";

        let result = get_sum_of_all_engine_gear_ratios(engine_schematic);

        assert_eq!(result, 467835);
    }
}

/// Gets the sum of all the engine part numbers from the schematic.
pub fn get_sum_of_all_engine_part_numbers(lift_engine_schematic: &str) -> u64 {
    let g = GondolaLiftEngine::from(lift_engine_schematic);
    let p = g.get_possible_parts();
    //println!("{p:#?}");
    let l: Vec<_> = p
        .iter()
        .filter_map(|part| match part.get_adjacent_symbols().is_empty() {
            true => None,
            _ => Some((part.number, part.get_adjacent_symbols())),
        })
        .collect();
    //println!("=== adjacent symbols ===");
    //println!("{l:#?}");
    let pn: Vec<_> = l.iter().map(|tuple| tuple.0).collect();
    //println!("pn = {pn:#?}");
    pn.iter().sum()

    //     GondolaLiftEngine::from(lift_engine_schematic)
    //         .get_possible_parts()
    //         .iter()
    //         .filter_map(|part| match part.get_adjacent_symbols().is_empty() {
    //             true => None,
    //             _ => Some(part.number),
    //         })
    //         .sum()
}

/// Gondola lift engine.
struct GondolaLiftEngine {
    /// All the parts in the engine.
    possible_parts: Vec<Part>,
}

impl GondolaLiftEngine {
    /// Gets possible parts.
    fn get_possible_parts(&self) -> &Vec<Part> {
        &self.possible_parts
    }
}

fn is_digit(v: Option<&str>) -> bool {
    matches!(
        v,
        Some("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
    )
}

/// Not actually needed - went down a rabbit hole trying to find the bug in my implementation with this one
fn is_sign(v: Option<&str>) -> bool {
    //matches!(v, Some("+" | "-"))
    false
}

impl From<&str> for GondolaLiftEngine {
    fn from(value: &str) -> Self {
        let value = value.replace(' ', "");
        let value = value.trim();
        let width = value
            .find('\n')
            .expect("GondolaLiftEngine::from<&str>: Unable to find first newline.");
        //println!("width = {width}");
        let lines = value.lines();
        let height = lines.clone().count();
        let buffer = lines.collect::<Vec<_>>().join("");
        //println!("buffer = {buffer}");
        let get_start = |x, y| -> Option<usize> {
            let offset = y as i32 * (width as i32) + x as i32;
            if offset < 0 {
                return None;
            }
            Some(offset as usize)
        };
        let get_thing = |x, y| {
            if let Some(start) = get_start(x, y) {
                return buffer.get(start..start + 1);
            }
            None
        };
        //println!("{:?}", get_thing(2, 2));

        let mut possible_parts = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(current_thing) = get_thing(x, y) {
                    if (is_sign(Some(current_thing)) && is_digit(get_thing(x + 1, y)))
                        || is_digit(Some(current_thing))
                    {
                        let left_thing = match x {
                            0 => None,
                            _ => get_thing(x - 1, y),
                        };
                        if !is_digit(left_thing) {
                            //println!(">>> new number {current_thing:?} at ({x}, {y})");
                            // This is a new part we need to catalog.
                            let number_slice_start = get_start(x, y).expect("GondolaLiftEngine::from<&str>: get_start should have returned some value.");
                            let number_slice_end = {
                                let mut sx = x + 1;
                                let mut done = false;
                                while !done {
                                    if sx >= width {
                                        done = true;
                                    } else {
                                        let s = get_thing(sx, y);
                                        //println!("symbol = {s:?}");
                                        if is_digit(s) {
                                            //println!("{s:?} is a digit");
                                            sx += 1;
                                        } else {
                                            //println!("DONE, [{number_slice_start}, {sx}]");
                                            done = true;
                                        }
                                    }
                                }
                                get_start(sx, y).expect("GondolaLiftEngine::from<&str>: get_start should always unwrap here.")
                            };
                            let number_slice = &buffer[number_slice_start..number_slice_end];
                            //println!("attempting to parse {number_slice}");
                            let number = number_slice
                                .parse::<u64>()
                                .expect("GondolaLiftEngine::from<&str>: unable to parse number");
                            let number_digits = number_slice.len();

                            // Create a vector with adjacent symbols (initialized with edge cases.)
                            let owned_optional_value = |v: &str| Some(v.to_owned());
                            let mut adjacent_things = vec![
                                // Top-left.
                                match (x, y) {
                                    (0, _) => None,
                                    (_, 0) => None,
                                    _ => get_thing(x - 1, y - 1).and_then(owned_optional_value),
                                },
                                // Left.
                                left_thing.and_then(owned_optional_value),
                                // Bottom-left.
                                match x {
                                    0 => None,
                                    _ => get_thing(x - 1, y + 1).and_then(owned_optional_value),
                                },
                                // Top-right.
                                match y {
                                    0 => None,
                                    _ => get_thing(x + number_digits, y - 1)
                                        .and_then(owned_optional_value),
                                },
                                // Right.
                                get_thing(x + number_digits, y).and_then(owned_optional_value),
                                // Bottom-right.
                                get_thing(x + number_digits, y + 1).and_then(owned_optional_value),
                            ];

                            // Fill in adjacent symbols with top and bottom symbols.
                            for ax in 0..number_digits {
                                // Top.
                                adjacent_things.push(match y {
                                    0 => None,
                                    _ => get_thing(x + ax, y - 1).and_then(owned_optional_value),
                                });
                                // Bottom.
                                adjacent_things
                                    .push(get_thing(x + ax, y + 1).and_then(owned_optional_value));
                            }

                            possible_parts.push(Part {
                                number,
                                adjacent_things,
                            });
                        }
                    }
                }
            }
        }

        Self { possible_parts }
    }
}

/// A part in the engine.
#[derive(Debug)]
struct Part {
    /// The part number.
    number: u64,

    /// Adjacent things.
    adjacent_things: Vec<Option<String>>,
}

impl Part {
    /// Gets adjacent symbols for the possible part.
    fn get_adjacent_symbols(&self) -> Vec<&str> {
        self.adjacent_things
            .iter()
            .filter_map(|thing| match thing {
                None => None,
                Some(t) => {
                    // if is_digit(thing.as_deref()) || t == "." {
                    if t == "." {
                        None
                    } else {
                        Some(t.as_str())
                    }
                }
            })
            .collect()
    }
}
