mod gondola_lift_engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please supply an input file as the first argument");
    let input = std::fs::read_to_string(&args[1])
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err:?}"));
    let value = gondola_lift_engine::get_sum_of_all_engine_part_numbers(&input);
    println!("Sum of part numbers = {value}");
}
