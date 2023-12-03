mod cube_game;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please supply an input file as the first argument");
    let input = std::fs::read_to_string(&args[1])
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err:?}"));
    let value = cube_game::get_possible_game_ids_sum(
        &input,
        cube_game::CubeSet {
            red: Some(12),
            green: Some(13),
            blue: Some(14),
        },
    );
    println!("Sum of game IDs = {value}");
}
