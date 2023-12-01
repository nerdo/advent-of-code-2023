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

enum Position {
    Beginning,
    End,
}

fn get_digit(line: &str, starting_from: Position) -> &str {
    let num_letters = line.len();
    let mut index = match starting_from {
        Position::Beginning => 0,
        Position::End => std::cmp::max(0, num_letters - 1),
    };

    loop {
        match starting_from {
            Position::Beginning => {
                if index >= num_letters {
                    return "";
                }
            }
            Position::End => {
                if index == 0 {
                    return "";
                }
            }
        };
        let current = &line[index..std::cmp::min(num_letters, index + 1)];
        match current {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => return current,
            _ => {}
        };

        let three_letter_word = &line[index..std::cmp::min(num_letters, index + 3)];
        let four_letter_word = &line[index..std::cmp::min(num_letters, index + 4)];
        let five_letter_word = &line[index..std::cmp::min(num_letters, index + 5)];

        if three_letter_word == "one" || three_letter_word == "two" || three_letter_word == "six" {
            match three_letter_word {
                "one" => return "1",
                "two" => return "2",
                "six" => return "6",
                _ => panic!("matched on a three letter word, but it wasn't one, two, or six!"),
            };
        } else if four_letter_word == "four"
            || four_letter_word == "five"
            || four_letter_word == "nine"
        {
            match four_letter_word {
                "four" => return "4",
                "five" => return "5",
                "nine" => return "9",
                _ => panic!("matched on a four letter word, but it wasn't four, five, or nine!"),
            };
        } else if five_letter_word == "three"
            || five_letter_word == "seven"
            || five_letter_word == "eight"
        {
            match five_letter_word {
                "three" => return "3",
                "seven" => return "7",
                "eight" => return "8",
                _ => panic!("matched on a five letter word, but it wasn't three, seven, or eight!"),
            };
        }
        match starting_from {
            Position::Beginning => index += 1,
            Position::End => index -= 1,
        };
    }
}

pub fn get_calibration_sum_from_spelled_out_digits(calibration_document: &str) -> i32 {
    let transformed_calibration_document = calibration_document
        .lines()
        .map(|line| {
            let first_digit = get_digit(line, Position::Beginning);
            let last_digit = get_digit(line, Position::End);
            format!("{first_digit}{last_digit}")
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
