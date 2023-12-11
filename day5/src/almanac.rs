//! Island Island Almanac
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day5_get_lowest_location_number_that_corresponds_to_any_initial_seed_number_returns_the_correct_answer(
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

        let result =
            get_lowest_location_number_that_corresponds_to_any_initial_seed_number(almanac_text);

        assert_eq!(result, 35);
    }
}

/// The Island Island Almanac
pub struct AlmanacBuilder {
    text: Option<String>,
}

// The backing store for almanac data.
struct AlmanacDatabase {
    seeds: HashSet<u64>,
    seed_to_soil: Buoy<Vec<Mapping>>,
    soil_to_fertilizer: Buoy<Vec<Mapping>>,
    fertilizer_to_water: Buoy<Vec<Mapping>>,
    water_to_light: Buoy<Vec<Mapping>>,
    light_to_temperature: Buoy<Vec<Mapping>>,
    temperature_to_humidity: Buoy<Vec<Mapping>>,
    humidity_to_location: Buoy<Vec<Mapping>>,
}

/// Generic wrapper tuple struct for newtype pattern for the sake of being able to "buoy" externally defined
/// types so that traits can be implemented for them.
struct Buoy<T>(pub T);

impl Buoy<Vec<Mapping>> {
    fn get(&self, source_value: u64) -> u64 {
        for mapping in &self.0 {
            if source_value >= mapping.source && source_value <= mapping.source + mapping.count {
                let offset = source_value - mapping.source;
                return mapping.destination + offset;
            }
        }
        source_value
    }
}

struct Mapping {
    source: u64,
    destination: u64,
    count: u64,
}

impl From<&str> for AlmanacDatabase {
    fn from(value: &str) -> Self {
        let seeds = HashSet::new();
        let seed_to_soil = Buoy(Vec::new());
        let soil_to_fertilizer = Buoy(Vec::new());
        let fertilizer_to_water = Buoy(Vec::new());
        let water_to_light = Buoy(Vec::new());
        let light_to_temperature = Buoy(Vec::new());
        let temperature_to_humidity = Buoy(Vec::new());
        let humidity_to_location = Buoy(Vec::new());

        let mut instance = Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };

        instance.seed(value);

        instance
    }
}

impl AlmanacDatabase {
    fn seed(&mut self, almanac_text: &str) {
        {
            let mut current_section: Option<AlmanacSection> = None;
            // Call to replace puts seed data on a new line to make it just like other data.
            for line in almanac_text.replace("seeds:", "seeds:\r\n").lines() {
                if line.trim().is_empty() {
                    continue;
                }

                let incoming = AlmanacTextParser::from(line);

                match (&current_section, incoming) {
                    (_, AlmanacTextParser::Section(new_section)) => {
                        current_section = Some(new_section);
                    }
                    (Some(section), AlmanacTextParser::Data(data)) => {
                        self.seed_section(section, &data);
                    }
                    (a, b) => panic!("Unexpected parser state: ({a:#?}, {b:#?})"),
                };
            }
        }
    }

    fn seed_section(&mut self, section: &AlmanacSection, numbers: &Vec<u64>) {
        match section {
            AlmanacSection::Seeds => {
                numbers.iter().for_each(|n| {
                    self.seeds.insert(*n);
                });
            }
            map_type => {
                if numbers.len() < 3 {
                    panic!("numbers should have at least 3 elements!");
                }
                let map = match &map_type {
                    AlmanacSection::Seeds => panic!("impossibru!"),
                    AlmanacSection::SeedToSoil => &mut self.seed_to_soil,
                    AlmanacSection::SoilToFertilizer => &mut self.soil_to_fertilizer,
                    AlmanacSection::FertilizerToWater => &mut self.fertilizer_to_water,
                    AlmanacSection::WaterToLight => &mut self.water_to_light,
                    AlmanacSection::LightToTemperature => &mut self.light_to_temperature,
                    AlmanacSection::TemperatureToHumidity => &mut self.temperature_to_humidity,
                    AlmanacSection::HumidityToLocation => &mut self.humidity_to_location,
                };
                let source = *numbers.get(1).expect("unable to get numbers[1]");
                let destination = *numbers.get(0).expect("unable to get numbers[0]");
                let count = *numbers.get(2).expect("unable to get numbers[2]");
                map.0.push(Mapping {
                    source,
                    destination,
                    count,
                });
            }
        }
    }
}

impl Default for AlmanacBuilder {
    fn default() -> Self {
        Self { text: None }
    }
}
impl AlmanacBuilder {
    /// Creates a new instance of an AlamancBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text for the AlmanacBuilder to parse when it builds.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = Some(text.into());
        self
    }

    /// Builds an Almanac from the text.
    pub fn build(&self) -> Result<Almanac, AlmanacBuilderError> {
        let Some(text) = self.text.as_ref() else {
            return Err(AlmanacBuilderError::NoText);
        };

        let db = AlmanacDatabase::from(text.as_str());

        Ok(Almanac { db })
    }
}

#[derive(Debug, Clone)]
enum AlmanacTextParser {
    Section(AlmanacSection),
    Data(Vec<u64>),
}

impl From<&str> for AlmanacTextParser {
    fn from(value: &str) -> Self {
        let text = value.trim();
        match text {
            "seeds:" => Self::Section(AlmanacSection::Seeds),
            "seed-to-soil map:" => Self::Section(AlmanacSection::SeedToSoil),
            "soil-to-fertilizer map:" => Self::Section(AlmanacSection::SoilToFertilizer),
            "fertilizer-to-water map:" => Self::Section(AlmanacSection::FertilizerToWater),
            "water-to-light map:" => Self::Section(AlmanacSection::WaterToLight),
            "light-to-temperature map:" => Self::Section(AlmanacSection::LightToTemperature),
            "temperature-to-humidity map:" => Self::Section(AlmanacSection::TemperatureToHumidity),
            "humidity-to-location map:" => Self::Section(AlmanacSection::HumidityToLocation),
            t => Self::Data(
                t.split_whitespace()
                    .map(|n| n.parse::<u64>().expect("Error parsing numbers"))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
enum AlmanacSection {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

/// Errors when building the almanac.
#[derive(Debug)]
pub enum AlmanacBuilderError {
    /// No text in the input.
    NoText,

    /// A database error occurred.
    Database(String),
}

/// The Island Island Almanac
pub struct Almanac {
    db: AlmanacDatabase,
}

impl Almanac {
    fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(&self) -> u64 {
        let db = &self.db;
        let mut locations = db
            .seeds
            .iter()
            .map(|seed| db.seed_to_soil.get(*seed))
            .map(|soil| db.soil_to_fertilizer.get(soil))
            .map(|fertilizer| db.fertilizer_to_water.get(fertilizer))
            .map(|water| db.water_to_light.get(water))
            .map(|light| db.light_to_temperature.get(light))
            .map(|temperature| db.temperature_to_humidity.get(temperature))
            .map(|humidity| db.humidity_to_location.get(humidity))
            .collect::<Vec<_>>();
        locations.sort_unstable();
        *locations
            .first()
            .expect("There should be at least one location!")
    }
}

/// Gets the lowest location number that corresonds to any initial seed number.
pub fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(
    almanac_text: &str,
) -> u64 {
    let almanac = AlmanacBuilder::new()
        .text(almanac_text)
        .build()
        .expect("Unable to build almanac");

    almanac.get_lowest_location_number_that_corresponds_to_any_initial_seed_number()
}
