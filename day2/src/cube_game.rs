//! Dice game
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn get_possible_game_ids_sum_returns_the_correct_answer() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let result = get_possible_game_ids_sum(
            input,
            CubeSet {
                red: Some(12),
                green: Some(13),
                blue: Some(14),
            },
        );

        assert_eq!(result, 8)
    }

    #[test]
    fn get_sum_of_power_of_minimum_cube_sets_returns_the_correct_answer() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let result = get_sum_of_power_of_minimum_cube_sets(input);

        assert_eq!(result, 2286)
    }
}

/// Defines a set of cubes
#[derive(Clone, Debug)]
pub struct CubeSet {
    /// The number of red cubes.
    pub red: Option<u32>,

    /// The number of green cubes.
    pub green: Option<u32>,

    /// The number of blue cubes.
    pub blue: Option<u32>,
}

impl CubeSet {
    pub fn get_power(&self) -> u32 {
        let mut power = 1;
        if let Some(red) = self.red {
            power *= red;
        }
        if let Some(green) = self.green {
            power *= green;
        }
        if let Some(blue) = self.blue {
            power *= blue;
        }
        power
    }
}

/// Gets the sum of the possible game IDs from the input if the bag were to only contain the specified cubes.
pub fn get_possible_game_ids_sum(game_records: &str, max_cubes: CubeSet) -> u32 {
    let games = Games::from(game_records);
    let games = games.get_possible_with_cubes(max_cubes);
    games.0.iter().map(|g| g.id).sum()
}

/// Newtype pattern to "buoy" external types for the sake of defining traits.
#[derive(Debug)]
struct Buoy<T>(pub T);

/// A vector of games.
type Games = Buoy<Vec<Game>>;

/// A game.
#[derive(Clone, Debug)]
struct Game {
    /// ID of the game.
    id: u32,

    /// Revealed cube sets.
    cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn get_minimum_cube_set(&self) -> CubeSet {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for c in &self.cube_sets {
            if let Some(r) = c.red {
                if r > red {
                    red = r;
                }
            }
            if let Some(g) = c.green {
                if g > green {
                    green = g;
                }
            }
            if let Some(b) = c.blue {
                if b > blue {
                    blue = b;
                }
            }
        }

        CubeSet {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
        }
    }
}

impl From<&str> for Games {
    fn from(value: &str) -> Self {
        let container = value
            .lines()
            .filter_map(|line| {
                if line.trim().is_empty() {
                    return None;
                }
                Some(Game::from(line.trim()))
            })
            .collect::<Vec<_>>();
        Buoy(container)
    }
}

impl Games {
    /// Returns the possible games if the bag contained only the given cubes.
    pub fn get_possible_with_cubes(&self, max_cubes: CubeSet) -> Self {
        let possible = self
            .0
            .iter()
            .filter(|g| {
                let red_limit = match max_cubes.red {
                    Some(n) => n,
                    None => 0,
                };
                let green_limit = match max_cubes.green {
                    Some(n) => n,
                    None => 0,
                };
                let blue_limit = match max_cubes.blue {
                    Some(n) => n,
                    None => 0,
                };
                for c in &g.cube_sets {
                    if let Some(red) = c.red {
                        if red > red_limit {
                            return false;
                        }
                    }
                    if let Some(green) = c.green {
                        if green > green_limit {
                            return false;
                        }
                    }
                    if let Some(blue) = c.blue {
                        if blue > blue_limit {
                            return false;
                        }
                    }
                }
                true
            })
            .cloned()
            .collect::<Vec<_>>();
        Buoy(possible)
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (id_part, record_part) = value
            .split_once(":")
            .expect("Game::from parse error: unable to split on :");
        let id = id_part[5..]
            .parse::<u32>()
            .expect("Game::from parse error: unable to parse ID");
        let cube_sets: Vec<CubeSet> = record_part
            .split(';')
            .into_iter()
            .map(|r| CubeSet::from(r.trim()))
            .collect();
        Game { id, cube_sets }
    }
}

impl From<&str> for CubeSet {
    fn from(value: &str) -> Self {
        value.split(", ").into_iter().fold(
            CubeSet {
                red: None,
                green: None,
                blue: None,
            },
            |mut cube_set, num_color_pair| {
                let (num_part, color_part) = num_color_pair
                    .split_once(' ')
                    .expect("CubeSet parse error: unable to split cube set with whitespace");

                let num = num_part
                    .trim()
                    .parse::<u32>()
                    .expect("CubeSet parse error: unable to parse number");
                match color_part.trim() {
                    "red" => cube_set.red = Some(num),
                    "green" => cube_set.green = Some(num),
                    "blue" => cube_set.blue = Some(num),
                    _ => panic!("Unexpected color: {color_part}"),
                };
                cube_set
            },
        )
    }
}

/// Gets the sum of the power of the minimum cube sets.
pub fn get_sum_of_power_of_minimum_cube_sets(game_records: &str) -> u32 {
    let games = Games::from(game_records);
    games
        .0
        .iter()
        .map(|g| g.get_minimum_cube_set().get_power())
        .sum()
}
