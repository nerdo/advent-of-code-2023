pub mod almanac;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please supply an input file as the first argument");
    let input = std::fs::read_to_string(&args[1])
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err:?}"));
    let value =
        almanac::get_lowest_location_number_that_corresponds_to_any_initial_seed_number(&input)
            .await;
    println!(
        "Lowest location number that corresponds to any of the initial seed numbers = {value}"
    );
}
