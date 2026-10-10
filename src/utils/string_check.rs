use crate::types::MonoError;
use crate::utils::get_paste;

pub fn string_check(text: Option<String>, paste: bool) -> Result<String, MonoError> {
    match text {
        Some(t) => Ok(t),
        None if paste => Ok(get_paste()?),
        None => Err(MonoError::InvalidInput(
            "Please provide a valid string when making a note".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_given_text() {
        let result = string_check(Some("hello".to_string()), false).unwrap();

        assert_eq!(result, "hello");
    }

    #[test]
    fn rejects_missing_text_when_not_pasting() {
        let result = string_check(None, false);

        assert!(matches!(result, Err(MonoError::InvalidInput(_))));
    }
}
