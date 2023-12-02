use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut valid_numbers: Vec<u32> = Vec::new();
    for line in _input.lines() {
        println!("Line: {}", line);
        let number = line
            .chars()
            .filter_map(|char| match char.is_numeric() {
                true => Some(char),
                false => None,
            })
            .collect::<String>();
        println!("Extracted number {}", number);
        if number.len() >= 1 {
            let first_character: char = number.chars().next().unwrap();
            let last_character: char = number.chars().next_back().unwrap();
            let concat_numer: u32 = format!("{}{}", first_character, last_character)
                .parse::<u32>()
                .unwrap();
            valid_numbers.push(concat_numer)
        }
    }

    println!("{:?}", &valid_numbers);

    let calibration: u32 = valid_numbers.iter().sum();

    Ok(calibration.to_string())
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
