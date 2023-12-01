use crate::custom_error::AocError;

pub fn process(_input : &str) -> miette::Result<String, AocError> {
    todo!("Do the processing stuff here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}