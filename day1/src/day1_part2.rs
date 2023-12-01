mod trebuchet_calibration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please supply an input file as the first argument");
    let input = std::fs::read_to_string(&args[1])
        .unwrap_or_else(|err| panic!("Error reading file '{file_path}': {err:?}"));
    let value = trebuchet_calibration::get_calibration_sum_from_spelled_out_digits(&input);
    println!("trebuchet calibration value = {value}");
}
