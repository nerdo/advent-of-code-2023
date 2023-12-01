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

        let result = get_calibration_sum(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn get_calibration_sum_from_spelled_out_digits_returns_the_correct_value() {
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        ";

        let result = get_calibration_sum_from_spelled_out_digits(input);

        assert_eq!(result, 281);
    }
}

pub fn get_calibration_sum_from_spelled_out_digits(calibration_document: &str) -> i32 {
    let transformed_calibration_document = calibration_document
        .lines()
        .map(|line| {
            let num_letters = line.len();
            let mut index = 0;
            let mut transformed_letters = Vec::new();
            while index < num_letters {
                let three_letter_word = &line[index..std::cmp::min(num_letters, index + 3)];
                let four_letter_word = &line[index..std::cmp::min(num_letters, index + 4)];
                let five_letter_word = &line[index..std::cmp::min(num_letters, index + 5)];

                if three_letter_word == "one"
                    || three_letter_word == "two"
                    || three_letter_word == "six"
                {
                    transformed_letters.push(String::from(match three_letter_word {
                        "one" => "1",
                        "two" => "2",
                        "six" => "6",
                        _ => three_letter_word,
                    }));
                    index += 3;
                } else if four_letter_word == "zero"
                    || four_letter_word == "four"
                    || four_letter_word == "five"
                    || four_letter_word == "nine"
                {
                    transformed_letters.push(String::from(match four_letter_word {
                        "zero" => "0",
                        "four" => "4",
                        "five" => "5",
                        "nine" => "9",
                        _ => four_letter_word,
                    }));
                    index += 4;
                } else if five_letter_word == "three"
                    || five_letter_word == "seven"
                    || five_letter_word == "eight"
                {
                    transformed_letters.push(String::from(match five_letter_word {
                        "three" => "3",
                        "seven" => "7",
                        "eight" => "8",
                        _ => five_letter_word,
                    }));

                    index += 5;
                } else {
                    transformed_letters.push(String::from(&line[index..index + 1]));
                    index += 1;
                }
            }
            transformed_letters.join("")
        })
        .collect::<Vec<_>>()
        .join("\n");
    get_calibration_sum(&transformed_calibration_document)
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
            if digits.is_empty() {
                return None;
            }
            Some(digits.first().unwrap() * 10 + digits.last().unwrap())
        })
        .sum()
}
