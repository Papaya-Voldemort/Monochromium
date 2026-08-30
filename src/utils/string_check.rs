use crate::utils::get_paste;

pub fn string_check(text: Option<String>, paste: bool) -> String {
    let full_text: String;
    if text == None {
        if paste {
            full_text = get_paste();
        } else {
            return "Please provide a message when making your note!".to_string();
        }
    } else {
        full_text = text.clone().unwrap();
    }
    
    full_text
}