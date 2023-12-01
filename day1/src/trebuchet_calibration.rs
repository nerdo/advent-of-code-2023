#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_calibration_sum_returns_the_correct_value() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        let result = get_calibration_sum(&input);

        assert_eq!(result, 142);
    }
}

pub fn get_calibration_sum(calibration_document: &str) -> i32 {
    calibration_document
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let digits = line
                .chars()
                .filter_map(|c| match c {
                    '0'..='9' => format!("{c}").parse::<i32>().ok(),
                    _ => None,
                })
                .collect::<Vec<_>>();
            Some(digits.first().unwrap() * 10 + digits.last().unwrap())
        })
        .sum()
}
