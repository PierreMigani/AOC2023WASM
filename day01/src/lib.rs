use wasm_bindgen::prelude::*;
use std::fmt;

#[derive(Copy, Clone)]
enum Numbers {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(PartialEq)]
enum NumberMatchs {
    PartialMatch,
    FullMatch(u32),
    NoMatch,
}

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Numbers::One => write!(f, "one"),
            Numbers::Two => write!(f, "two"),
            Numbers::Three => write!(f, "three"),
            Numbers::Four => write!(f, "four"),
            Numbers::Five => write!(f, "five"),
            Numbers::Six => write!(f, "six"),
            Numbers::Seven => write!(f, "seven"),
            Numbers::Eight => write!(f, "eight"),
            Numbers::Nine => write!(f, "nine"),
        }
    }
}

impl Numbers {
    fn match_str(&self, input: &Vec<char>) -> NumberMatchs {
        if input.len() == 0 {
            return NumberMatchs::NoMatch;
        }

        let number_string = self.to_string();
        let mut number_chars = number_string.chars();
        for c in input {
            if Some(*c) != number_chars.next() {
                return NumberMatchs::NoMatch;
            }
        }
        
        if number_string.len() == input.len() {
            // cast to u32
            return NumberMatchs::FullMatch(*self as u32);
        }

        NumberMatchs::PartialMatch
    }
}

fn find_match_in_numbers(input: &Vec<char>) -> NumberMatchs {
    let numbers = vec![
        Numbers::One,
        Numbers::Two,
        Numbers::Three,
        Numbers::Four,
        Numbers::Five,
        Numbers::Six,
        Numbers::Seven,
        Numbers::Eight,
        Numbers::Nine,
    ];

    let mut match_found = NumberMatchs::NoMatch;
    for number in &numbers {
        match_found = number.match_str(input);
        if match_found != NumberMatchs::NoMatch {
            break;
        }
    }

    match_found
}

fn decode_calibration(input: &str, convert_string_numbers: bool) -> Result<u32, std::num::ParseIntError> {
    let mut digits = String::new();
    let mut saved_string_number_value = Vec::new();

    // loop over each character in the input string
    input.chars().for_each(|c| {
        // if the character is a digit, add it to the digits string
        if c.is_digit(10) {
            digits.push(c);
            saved_string_number_value.clear();
        } else if convert_string_numbers {
            // find a match for the current saved characters
            saved_string_number_value.push(c);
            let mut match_found = find_match_in_numbers(&saved_string_number_value);

            if let NumberMatchs::NoMatch = match_found {
                // remove saved first characters until we find a match or no saved characters are left
                while match_found == NumberMatchs::NoMatch && saved_string_number_value.len() > 0 {
                    saved_string_number_value.remove(0);
                    match_found = find_match_in_numbers(&saved_string_number_value);
                }
            }

            if let NumberMatchs::FullMatch(number) = match_found {
                digits.push_str(&number.to_string());
                saved_string_number_value.clear();
            }
        }
    });

    // get first and last digits
    let digit_chars = digits.chars().collect::<Vec<char>>();
    let first_digit = digit_chars.get(0).unwrap_or(&'0');
    let last_digit = digit_chars.get(digit_chars.len() - 1).unwrap_or(&'0');

    // form a string with the first and last digits
    let mut calibration_value = String::new();
    calibration_value.push(*first_digit);
    calibration_value.push(*last_digit);

    calibration_value.parse::<u32>()
}

fn sum_calibrations(lines: Vec<&str>, convert_numbers: bool) -> u32 {
    let mut sum = 0;
    for line in lines {
        sum += decode_calibration(line, convert_numbers).expect("Could not parse calibration");
    }
    sum
}

#[wasm_bindgen]
pub fn compute_part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    sum_calibrations(lines, false)
}

#[wasm_bindgen]
pub fn compute_part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    sum_calibrations(lines, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_calibration_without_convertion() {
        assert_eq!(decode_calibration("1abc2", false), Result::Ok(12));
        assert_eq!(decode_calibration("pqr3stu8vwx", false), Result::Ok(38));
        assert_eq!(decode_calibration("a1b2c3d4e5f", false), Result::Ok(15));
        assert_eq!(decode_calibration("treb7uchet", false), Result::Ok(77));
    }

    #[test]
    fn test_decode_calibration_with_convertion() {
        assert_eq!(decode_calibration("two1nine", true), Result::Ok(29));
        assert_eq!(decode_calibration("eightwothree", true), Result::Ok(83));
        assert_eq!(decode_calibration("abcone2threexyz", true), Result::Ok(13));
        assert_eq!(decode_calibration("xtwone3four", true), Result::Ok(24));
        assert_eq!(decode_calibration("4nineeightseven2", true), Result::Ok(42));
        assert_eq!(decode_calibration("zoneight234", true), Result::Ok(14));
        assert_eq!(decode_calibration("7pqrstsixteen", true), Result::Ok(76));
    }

    #[test]
    fn test_sum_calibrations_without_convertion() {
        let lines = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        assert_eq!(sum_calibrations(lines, false), 142);
    }

    #[test]
    fn test_sum_calibrations_with_convertion() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(sum_calibrations(lines, true), 281);
    }
}


