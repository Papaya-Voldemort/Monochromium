use crate::types::MonoError;
use crate::utils::get_paste;

pub fn string_check(text: Option<String>, paste: bool) -> Result<String, MonoError> {
    match text {
        Some(t) => Ok(t),
        None if paste => Ok(get_paste()?),
        None => Ok("Please provide a message when making your note!".to_string()),
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
    fn returns_error_message_when_no_text_and_no_paste() {
        let result = string_check(None, false).unwrap();

        assert_eq!(result, "Please provide a message when making your note!");
    }
}
