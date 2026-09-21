use crate::types::Note;

pub fn pretty_notes(notes: Vec<Note>, view: bool) -> Vec<String> {
    let mut output = Vec::new();

    if view {
        for note in notes {
            let push = format!(
                "{} \u{2022} ID: {} \u{2022} {}\n {}",
                note.title,
                note.id,
                note.date.format("%b %d, %Y at%l:%M %p"),
                note.content
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_pretty_notes_formating_with_view() {
        let test_notes = vec![
            Note {
                id: 1,
                title: "First test note".to_string(),
                _note_type: "note".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 21)
                    .unwrap()
                    .and_hms_opt(9, 30, 0)
                    .unwrap(),
                content: "This is the content of the first note.".to_string(),
            },
            Note {
                id: 2,
                title: "Buy groceries".to_string(),
                _note_type: "todo".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 22)
                    .unwrap()
                    .and_hms_opt(16, 45, 0)
                    .unwrap(),
                content: "Milk, bread, eggs, and peanut butter.".to_string(),
            },
            Note {
                id: 3,
                title: "Morning check-in".to_string(),
                _note_type: "checkin".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 23)
                    .unwrap()
                    .and_hms_opt(7, 5, 0)
                    .unwrap(),
                content: "Feeling productive and ready to work.".to_string(),
            },
        ];

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
        let test_notes = vec![
            Note {
                id: 1,
                title: "First test note".to_string(),
                _note_type: "note".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 21)
                    .unwrap()
                    .and_hms_opt(9, 30, 0)
                    .unwrap(),
                content: "This is the content of the first note.".to_string(),
            },
            Note {
                id: 2,
                title: "Buy groceries".to_string(),
                _note_type: "todo".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 22)
                    .unwrap()
                    .and_hms_opt(16, 45, 0)
                    .unwrap(),
                content: "Milk, bread, eggs, and peanut butter.".to_string(),
            },
            Note {
                id: 3,
                title: "Morning check-in".to_string(),
                _note_type: "checkin".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 9, 23)
                    .unwrap()
                    .and_hms_opt(7, 5, 0)
                    .unwrap(),
                content: "Feeling productive and ready to work.".to_string(),
            },
        ];

        let test_out = pretty_notes(test_notes, false);

        let expected = vec![
            "First test note • ID: 1".to_string(),
            "Buy groceries • ID: 2".to_string(),
            "Morning check-in • ID: 3".to_string(),
        ];

        assert_eq!(test_out, expected);
    }
}
