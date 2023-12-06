//! Island Island Almanac
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn day5_get_lowest_location_number_that_corresponds_to_any_initial_seed_number_returns_the_correct_answer(
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
            get_lowest_location_number_that_corresponds_to_any_initial_seed_number(almanac_text)
                .await;

        assert_eq!(result, 35);
    }
}

/// The Island Island Almanac
pub struct AlmanacBuilder {
    text: Option<String>,
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
    pub async fn build(&self) -> Result<Almanac, AlmanacBuilderError> {
        let Some(text) = self.text.as_ref() else {
            return Err(AlmanacBuilderError::NoText);
        };

        let db = Self::initialize(text).await;

        Ok(Almanac { db })
    }

    async fn initialize(almanac_text: &str) -> Surreal<Db> {
        let db = Self::connect_to_db().await;
        Self::seed_db(&db, almanac_text).await;
        db
    }

    async fn connect_to_db() -> Surreal<Db> {
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Unable to connect to SurrealDB");
        db.use_ns("aoc_almanac")
            .use_db("aoc_almaanc")
            .await
            .expect("Unable to establish namespace, database.");
        db
    }

    async fn seed_db(db: &Surreal<Db>, almanac_text: &str) {
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
                        Self::seed_section(db, section, &data).await;
                    }
                    (a, b) => panic!("Unexpected parser state: ({a:#?}, {b:#?})"),
                };
            }
        }
    }

    async fn seed_section(db: &Surreal<Db>, section: &AlmanacSection, numbers: &Vec<u64>) {
        match section {
            AlmanacSection::Seeds => {
                let sql = numbers
                    .iter()
                    .map(|n| format!("INSERT IGNORE INTO seed {{id: {n}}} RETURN NONE;"))
                    .collect::<Vec<_>>()
                    .join("");
                println!("{sql}");
                db.query(sql).await.expect("seeds: sql query error");
            }
            AlmanacSection::SeedToSoil => Self::relate(db, "seed", "soil", numbers).await,
            AlmanacSection::SoilToFertilizer => {
                Self::relate(db, "soil", "fertilizer", numbers).await
            }
            AlmanacSection::FertilizerToWater => {
                Self::relate(db, "fertilizer", "water", numbers).await
            }
            AlmanacSection::WaterToLight => Self::relate(db, "water", "light", numbers).await,
            AlmanacSection::LightToTemperature => {
                Self::relate(db, "light", "temperature", numbers).await
            }
            AlmanacSection::TemperatureToHumidity => {
                Self::relate(db, "temperature", "humidity", numbers).await
            }
            AlmanacSection::HumidityToLocation => {
                Self::relate(db, "humidity", "location", numbers).await
            }
        }
    }

    async fn relate(db: &Surreal<Db>, from: &str, to: &str, numbers: &Vec<u64>) {
        if numbers.len() < 3 {
            panic!("numbers should have at least 3 elements!");
        }

        let from_start = numbers.get(1).expect("unable to get numbers[1]");
        let to_start = numbers.get(0).expect("unable to get numbers[0]");
        let count = numbers.get(2).expect("unable to get numbers[2]");

        let sql = (0..*count)
            .map(|offset| {
                let from_id = from_start + offset;
                let to_id = to_start + offset;
                let insert_from =
                    format!("INSERT IGNORE INTO {from} {{id: {from_id}}} RETURN NONE");
                let insert_to = format!("INSERT IGNORE INTO {to} {{id: {to_id}}} RETURN NONE");
                let relation = format!("RELATE {from}:{from_id}->to->{to}:{to_id} RETURN NONE");
                format!("{insert_from};{insert_to};{relation};")
            })
            .collect::<Vec<_>>()
            .join("");

        println!("{sql}");
        db.query(sql).await.expect("relate: sql query error");
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
    NoText,
    Database(String),
}

/// The Island Island Almanac
pub struct Almanac {
    db: Surreal<Db>,
}

impl Almanac {
    async fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(&self) -> u64 {
        let results = self
            .db
            .query("SELECT humidity.out<-to<-location FROM seed")
            .await
            .expect("sql query error");
        // println!("{results:#?}");
        todo!()
    }
}

pub async fn get_lowest_location_number_that_corresponds_to_any_initial_seed_number(
    almanac_text: &str,
) -> u64 {
    let almanac = AlmanacBuilder::new()
        .text(almanac_text)
        .build()
        .await
        .expect("Unable to build almanac");

    almanac
        .get_lowest_location_number_that_corresponds_to_any_initial_seed_number()
        .await
}
