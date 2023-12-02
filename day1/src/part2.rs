use std::collections::HashMap;
use crate::custom_error::AocError;

fn apply_mapping(slice : String) -> Option<String> {
    let number_as_word: HashMap<&str, &str> = HashMap::from([
        ("one","1"),
        ("two","2"),
        ("three","3"),
        ("four","4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9"),
    ]);
    for (key,value) in number_as_word.clone().into_iter() {
        if slice.contains(key) {
            return Some(value.to_string())
        }
    }
    None
}

fn map_word_to_number(line : &str) -> Option<String> {
    // we need the first and last number
    // simply return on first hit
    let number: u32;
    for (i,current_character) in line.chars().enumerate() {
        if current_character.is_numeric() {
            number = current_character.to_digit(10).unwrap();
            return Some(number.to_string())
        }
        if i >= 2 {
            // check if a word is given
            let current_chars : String = line.chars().take(i+1).collect::<String>();
            match apply_mapping(current_chars) {
                Some(number) => {return Some(number)},
                None => continue
            }
        }
    }
    None
}

fn map_word_to_number_reverse(line : &str) -> Option<String> {
    // we need the first and last number
    // simply return on first hit
    for (i,current_character) in line.chars().rev().enumerate() {
        if current_character.is_numeric() {
            let number : u32 = current_character.to_digit(10).unwrap();
            return Some(number.to_string())
        }
        if i >= 2 {
            // check if a word is given
            let current_chars : String = line[line.len() - (i+1)..].parse().unwrap();
            match apply_mapping(current_chars) {
               Some(number) => {return Some(number)},
                None => continue
            }
        }
    }
    None
}

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut valid_numbers: Vec<u32> = Vec::new();
    for line in _input.lines() {
        let possible_number = map_word_to_number(line);
        let possible_number_reverse = map_word_to_number_reverse(line);
        match (possible_number,possible_number_reverse) {
            (Some(a),Some(b)) => {
                let concat_numer : u32 = format!("{}{}",a,b)
                    .parse::<u32>()
                    .unwrap();
                valid_numbers.push(concat_numer)
            }
            (_,_) => {
                continue
            }
        }
    }
    println!("Found the following numbers {:?}",&valid_numbers);
    let calibration_number: u32 = valid_numbers.iter().sum();
    println!("Calculated calibration number: {}", calibration_number);
    Ok(calibration_number.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
