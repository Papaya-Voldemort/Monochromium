use crate::commands::definitions::NoteTypes;

pub fn parse_note_type(note_type: NoteTypes) -> String {
    let note_type_string: String = match note_type {
        NoteTypes::Idea => String::from("idea"),
        NoteTypes::CheckIn => String::from("check in"),
        NoteTypes::Todo => String::from("todo"),
        NoteTypes::Other => String::from("other"),
    };

    note_type_string
}