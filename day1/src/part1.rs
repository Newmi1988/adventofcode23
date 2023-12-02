use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut valid_numbers: Vec<u32> = Vec::new();
    for line in _input.lines() {
        let number = line
            .chars()
            .filter(|char| char.is_numeric())
            .collect::<String>();
        if !number.is_empty() {
            let first_character: char = number.chars().next().unwrap();
            let last_character: char = number.chars().next_back().unwrap();
            let concat_numer: u32 = format!("{}{}", first_character, last_character)
                .parse::<u32>()
                .unwrap();
            valid_numbers.push(concat_numer)
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
