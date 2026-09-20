use crate::utils::get_paste;

pub fn string_check(text: Option<String>, paste: bool) -> String {
    match text {
        Some(t) => t,
        None if paste => get_paste(),
        None => "Please provide a message when making your note!".to_string(),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
// }
