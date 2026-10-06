use crate::types::Note;

pub fn pretty_notes(notes: Vec<Note>, view: bool) -> Vec<String> {
    notes
        .into_iter()
        .map(|note| {
            if view {
                format!(
                    "{} • ID: {} • {}\n {}",
                    note.title,
                    note.id,
                    note.date.format("%b %d, %Y at%l:%M %p"),
                    note.content.unwrap_or_default()
                )
            } else {
                format!("{} • ID: {}", note.title, note.id)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::NoteTypes;
    use chrono::NaiveDate;
    use std::path::PathBuf;

    fn test_notes() -> Vec<Note> {
        vec![
            Note {
                id: 1,
                title: "First test note".to_string(),
                note_type: NoteTypes::Other,
                date: NaiveDate::from_ymd_opt(2026, 9, 21)
                    .unwrap()
                    .and_hms_opt(9, 30, 0)
                    .unwrap(),
                file_link: PathBuf::new(),
                content: Some("This is the content of the first note.".to_string()),
            },
            Note {
                id: 2,
                title: "Buy groceries".to_string(),
                note_type: NoteTypes::Other,
                date: NaiveDate::from_ymd_opt(2026, 9, 22)
                    .unwrap()
                    .and_hms_opt(16, 45, 0)
                    .unwrap(),
                file_link: PathBuf::new(),
                content: Some("Milk, bread, eggs, and peanut butter.".to_string()),
            },
            Note {
                id: 3,
                title: "Morning check-in".to_string(),
                note_type: NoteTypes::Other,
                date: NaiveDate::from_ymd_opt(2026, 9, 23)
                    .unwrap()
                    .and_hms_opt(7, 5, 0)
                    .unwrap(),
                file_link: PathBuf::new(),
                content: Some("Feeling productive and ready to work.".to_string()),
            },
        ]
    }

    #[test]
    fn test_pretty_notes_formating_with_view() {
        let test_notes = test_notes();

        let test_out = pretty_notes(test_notes, true);

        let expected = vec![
            "First test note • ID: 1 • Sep 21, 2026 at 9:30 AM\n This is the content of the first note.".to_string(),
            "Buy groceries • ID: 2 • Sep 22, 2026 at 4:45 PM\n Milk, bread, eggs, and peanut butter.".to_string(),
            "Morning check-in • ID: 3 • Sep 23, 2026 at 7:05 AM\n Feeling productive and ready to work.".to_string(),
        ];

        assert_eq!(test_out, expected);
    }

    #[test]
    fn test_pretty_notes_formating_without_view() {
        let test_notes = test_notes();

        let test_out = pretty_notes(test_notes, false);

        let expected = vec![
            "First test note • ID: 1".to_string(),
            "Buy groceries • ID: 2".to_string(),
            "Morning check-in • ID: 3".to_string(),
        ];

        assert_eq!(test_out, expected);
    }
}
