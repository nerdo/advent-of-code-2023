//! Engine for gondola lift.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

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

/// Gets the sum of the gear ratios from the schematic.
pub fn get_sum_of_all_engine_gear_ratios(lift_engine_schematic: &str) -> u64 {
    let es = EngineSchematics::from(lift_engine_schematic);
    let gear_ratios = get_gear_ratios(&es);
    gear_ratios.iter().sum()
}

struct EngineSchematics {
    /// Width of the schematic.
    width: usize,

    /// Height of the schematic.
    height: usize,

    /// Buffer containing schematic details.
    buffer: String,
}

impl EngineSchematics {
    fn get_thing(&self, x: usize, y: usize) -> Option<&str> {
        if let Some(start) = self.get_start(x, y) {
            return self.buffer.get(start..start + 1);
        }
        None
    }

    fn get_start(&self, x: usize, y: usize) -> Option<usize> {
        let offset = y as i32 * (self.width as i32) + x as i32;
        if offset < 0 {
            return None;
        }
        Some(offset as usize)
    }

    fn get_numbers_adjacent_to(&self, x: usize, y: usize) -> Vec<u64> {
        // Left and right are the simplest cases since there is no potential for multiple numbers
        // on either side.
        let mut numbers = vec![
            // Left.
            match x {
                0 => None,
                _ => self.get_number(x - 1, y),
            },
            // Right.
            self.get_number(x + 1, y),
        ];

        let mut y_coords = vec![y + 1];
        if y > 0 {
            y_coords.push(y - 1);
        }

        for sy in y_coords {
            let sx = x + 1;
            let mut next_offset = 0;
            let mut maybe_more_numbers_to_the_left = true;
            while maybe_more_numbers_to_the_left {
                let number = match y {
                    0 => None,
                    _ => self.get_number(sx - next_offset, sy),
                };
                if let Some(number) = number {
                    numbers.push(Some(number));

                    // Start where the next number could be.
                    // +1 would get us one spot over, but that would mean it's part of the same
                    // number;
                    // +2 is the next possible spot.
                    next_offset += number.left_offset + 2
                } else {
                    next_offset += 1;
                }

                // Because there are 3 possible spots above/below the reference point.
                // But only 2 when we're all the way to the left.
                maybe_more_numbers_to_the_left = next_offset
                    < match x {
                        0 => 2,
                        _ => 3,
                    };
            }
        }

        numbers
            .iter()
            .filter_map(|n| match n {
                None => None,
                Some(number) => Some(number.value),
            })
            .collect::<Vec<_>>()
    }

    fn get_number(&self, x: usize, y: usize) -> Option<Number> {
        if !is_digit(self.get_thing(x, y)) {
            return None;
        }

        let mut sx = x;
        while sx > 0 && is_digit(self.get_thing(sx - 1, y)) {
            sx -= 1;
        }

        let slice_start = self
            .get_start(sx, y)
            .expect("EngineSchematics::get_number: get_start should have returned some value.");
        let slice_end = {
            let mut sx = x + 1;
            let mut done = false;
            while !done {
                if sx >= self.width {
                    done = true;
                } else {
                    let s = self.get_thing(sx, y);
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
            self.get_start(sx, y)
                .expect("EngineSchematics::get_number: get_start should always unwrap here.")
        };
        let slice = &self.buffer[slice_start..slice_end];
        //println!("attempting to parse {number_slice}");
        let number = slice
            .parse::<u64>()
            .expect("EngineSchematics::from<&str>: unable to parse number");
        let num_digits = slice.len();
        Some(Number {
            value: number,
            left_offset: x - sx,
            num_digits,
        })
    }
}

#[derive(Copy, Clone)]
struct Number {
    value: u64,
    left_offset: usize,
    #[allow(unused_variables, dead_code)]
    num_digits: usize,
}

impl From<&str> for EngineSchematics {
    fn from(value: &str) -> Self {
        let value = value.replace(' ', "");
        let value = value.trim();
        let width = value
            .find('\n')
            .expect("EngineSchematics::from<&str>: Unable to find first newline.");
        let lines = value.lines();
        let height = lines.clone().count();
        let buffer = lines.collect::<Vec<_>>().join("");
        Self {
            width,
            height,
            buffer,
        }
    }
}

fn get_gear_ratios(schematics: &EngineSchematics) -> Vec<u64> {
    let mut gear_ratios = Vec::new();

    for x in 0..schematics.width {
        for y in 0..schematics.height {
            if schematics.get_thing(x, y) == Some("*") {
                let adjacent_numbers = schematics.get_numbers_adjacent_to(x, y);
                if adjacent_numbers.len() == 2 {
                    gear_ratios.push(
                        adjacent_numbers.first().expect("impossibru")
                            * adjacent_numbers.last().expect("impossibru"),
                    );
                }
            }
        }
    }

    gear_ratios
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
fn is_sign(_v: Option<&str>) -> bool {
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
