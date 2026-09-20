use crate::types::Note;

pub fn pretty_notes(notes: Vec<Note>, view: bool) -> Vec<String> {
    let mut output = Vec::new();

    if view {
        for note in notes {
            let push = format!(
                "{} \u{2022} ID: {} \u{2022} {}\n {}",
                note.title, note.id, note.date, note.content
            );
            output.push(push);
        }
    } else {
        for note in notes {
            let push = format!("{} \u{2022} ID: {}", note.title, note.id);
            output.push(push);
        }
    }

    output
}
