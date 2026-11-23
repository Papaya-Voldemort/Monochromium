use super::{delete, edit, export, get_stats, handle_todos, list, search};
use crate::database::Database;
use crate::types::{EditMode, MonoError, NoteTypes, Score};
use chrono::{Duration, Local, NaiveDate};
use std::{fs, path::Path, path::PathBuf};

fn note(
    db: &Database,
    directory: &Path,
    title: &str,
    content: &str,
    kind: NoteTypes,
    date: NaiveDate,
) -> Result<u32, MonoError> {
    let path = directory.join(format!("{title}.md"));
    fs::write(&path, content)?;
    Ok(db.add_row(
        title.to_string(),
        kind,
        path,
        date.and_hms_opt(9, 30, 0).unwrap(),
        None,
    )?)
}

#[test]
fn search_matches_bodies_and_filters_before_limiting() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let yesterday = today - Duration::days(1);
    let id = note(
        &db,
        directory.path(),
        "older",
        "BodyOnlyWord",
        NoteTypes::Idea,
        yesterday,
    )?;
    note(
        &db,
        directory.path(),
        "newer",
        "Unrelated",
        NoteTypes::Idea,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "other",
        "BodyOnlyWord",
        NoteTypes::Other,
        today,
    )?;
    let results = search(
        &db,
        "bodyonlyword".to_string(),
        Some(1),
        Some(NoteTypes::Idea),
        None,
    )?;
    assert_eq!(results, [format!("{id:>3}  older")]);
    assert_eq!(
        search(&db, "BODYONLYWORD".to_string(), None, None, Some(yesterday))?,
        results
    );
    assert!(search(&db, "bodyonlyword".to_string(), Some(0), None, None)?.is_empty());
    Ok(())
}

#[test]
fn list_applies_today_and_since_filters() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    note(
        &db,
        directory.path(),
        "yesterday",
        "Old",
        NoteTypes::Other,
        today - Duration::days(1),
    )?;
    let id = note(
        &db,
        directory.path(),
        "today",
        "Current",
        NoteTypes::Other,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "tomorrow",
        "Future",
        NoteTypes::Other,
        today + Duration::days(1),
    )?;
    assert_eq!(
        list(&db, Some(1), None, true, None, false, false)?,
        [format!("{id:>3}  today")]
    );
    let results = list(&db, None, None, false, Some(today), false, false)?;
    assert_eq!(results.len(), 2);
    assert!(!results.iter().any(|line| line.contains("yesterday")));

    assert_eq!(
        list(
            &db,
            None,
            None,
            true,
            Some(today - Duration::days(1)),
            false,
            false
        )?,
        [format!("{id:>3}  today")]
    );
    assert!(
        list(
            &db,
            None,
            None,
            true,
            Some(today + Duration::days(1)),
            false,
            false
        )?
        .is_empty()
    );
    Ok(())
}

#[test]
fn deletion_failures_preserve_database_row_and_file() -> Result<(), MonoError> {
    for fail_commit in [false, true] {
        let db = Database::new_in_memory()?;
        let directory = tempfile::tempdir()?;
        let id = note(
            &db,
            directory.path(),
            "test",
            "Original body",
            NoteTypes::Other,
            Local::now().date_naive(),
        )?;
        if fail_commit {
            db.conn.execute_batch(
                "PRAGMA foreign_keys = ON;
                 CREATE TABLE dependencies (note_id INTEGER REFERENCES notes(id) DEFERRABLE INITIALLY DEFERRED);
                 INSERT INTO dependencies VALUES (1);",
            )?;
        } else {
            db.conn.execute_batch(
                "CREATE TRIGGER refuse_delete BEFORE DELETE ON notes
                 BEGIN SELECT RAISE(ABORT, 'simulated failure'); END;",
            )?;
        }
        assert!(delete(&db, vec![id], true).is_err());
        let row = db.read_single_row(id)?;
        assert_eq!(fs::read_to_string(row.file_link)?, "Original body");
        assert_eq!(fs::read_dir(directory.path())?.count(), 1);
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn batch_deletion_failures_preserve_every_row_and_file() -> Result<(), MonoError> {
    use std::os::unix::fs::PermissionsExt;

    for fail_commit in [false, true] {
        let db = Database::new_in_memory()?;
        let first_dir = tempfile::tempdir()?;
        let second_dir = tempfile::tempdir()?;
        let today = Local::now().date_naive();
        let first = note(
            &db,
            first_dir.path(),
            "one",
            "First",
            NoteTypes::Other,
            today,
        )?;
        let second = note(
            &db,
            second_dir.path(),
            "two",
            "Second",
            NoteTypes::Other,
            today,
        )?;

        if fail_commit {
            db.conn.execute_batch(
                "PRAGMA foreign_keys = ON;
                 CREATE TABLE dependencies (note_id INTEGER REFERENCES notes(id) DEFERRABLE INITIALLY DEFERRED);
                 INSERT INTO dependencies VALUES (2);",
            )?;
        } else {
            // The first backup succeeds; the second can't be created in a read-only directory.
            fs::set_permissions(second_dir.path(), fs::Permissions::from_mode(0o555))?;
        }

        let result = delete(&db, vec![first, second], true);
        fs::set_permissions(second_dir.path(), fs::Permissions::from_mode(0o755))?;

        assert!(result.is_err());
        for (id, dir, body) in [
            (first, first_dir.path(), "First"),
            (second, second_dir.path(), "Second"),
        ] {
            let row = db.read_single_row(id)?;
            assert_eq!(fs::read_to_string(row.file_link)?, body);
            assert_eq!(fs::read_dir(dir)?.count(), 1);
        }
    }
    Ok(())
}

#[test]
fn deletion_removes_notes_and_handles_missing_files() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    for missing in [false, true] {
        let id = note(
            &db,
            directory.path(),
            "test",
            "Body",
            NoteTypes::Other,
            Local::now().date_naive(),
        )?;
        if missing {
            fs::remove_file(db.read_single_row(id)?.file_link)?;
        }
        delete(&db, vec![id], true)?;
        assert!(db.read_single_row(id).is_err());
        assert_eq!(fs::read_dir(directory.path())?.count(), 0);
    }
    Ok(())
}

#[test]
fn editing_normalizes_types_and_rejects_missing_file_links() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let id = note(
        &db,
        directory.path(),
        "test",
        "Body",
        NoteTypes::Other,
        Local::now().date_naive(),
    )?;
    for alias in ["check-in", "checkin", "check in"] {
        edit(&db, id, Some(EditMode::Type), Some(alias.to_string()))?;
        assert_eq!(db.read_single_row(id)?.note_type, NoteTypes::CheckIn);
    }
    let before = db.read_single_row(id)?.file_link;
    assert!(
        edit(
            &db,
            id,
            Some(EditMode::FilePath),
            Some(
                directory
                    .path()
                    .join("missing.md")
                    .to_string_lossy()
                    .to_string()
            )
        )
        .is_err()
    );
    assert_eq!(db.read_single_row(id)?.file_link, before);
    Ok(())
}

#[test]
fn missing_files_do_not_abort_bulk_commands() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let missing = note(
        &db,
        directory.path(),
        "missing",
        "Gone",
        NoteTypes::Other,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "healthy",
        "Kept content",
        NoteTypes::Other,
        today,
    )?;
    fs::remove_file(db.read_single_row(missing)?.file_link)?;
    let output = export(&db)?;
    assert_eq!(output.len(), 1);
    assert!(output[0].contains("Kept content"));
    assert_eq!(list(&db, None, None, false, None, true, false)?.len(), 2);
    assert_eq!(
        search(&db, "Kept content".to_string(), None, None, None)?.len(),
        1
    );
    Ok(())
}

#[test]
fn export_includes_scores_in_note_metadata() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let mood_path = directory.path().join("mood.md");
    fs::write(&mood_path, "Rough day")?;
    db.add_row(
        "mood".to_string(),
        NoteTypes::CheckIn,
        mood_path,
        today.and_hms_opt(9, 30, 0).unwrap(),
        Some(Score::new(-3)?),
    )?;
    note(
        &db,
        directory.path(),
        "plain",
        "Unscored body",
        NoteTypes::Other,
        today,
    )?;

    let output = export(&db)?;
    let scored = output
        .iter()
        .find(|entry| entry.contains("## mood"))
        .unwrap();
    assert!(scored.contains("value: -3"));
    let unscored = output
        .iter()
        .find(|entry| entry.contains("## plain"))
        .unwrap();
    assert!(unscored.contains("value: None"));
    Ok(())
}

#[test]
fn value_flag_accepts_negative_and_rejects_out_of_range() {
    use crate::types::{MonoCLI, MonoCommands};
    use clap::Parser;

    let cli = MonoCLI::try_parse_from(["mono", "add", "text", "-v", "-3"]).unwrap();
    match cli.command {
        Some(MonoCommands::Add { value, .. }) => assert_eq!(value, Some(-3)),
        _ => panic!("expected add command"),
    }

    assert!(MonoCLI::try_parse_from(["mono", "add", "text", "-v", "6"]).is_err());
    assert!(MonoCLI::try_parse_from(["mono", "add", "text", "-v", "-6"]).is_err());
}

#[test]
fn sugar_commands_parse_scores_and_reject_out_of_range() {
    use crate::types::{MonoCLI, MonoCommands};
    use clap::Parser;

    let cli = MonoCLI::try_parse_from(["mono", "checkin", "text", "-m", "-3"]).unwrap();
    match cli.command {
        Some(MonoCommands::Checkin { mood, .. }) => assert_eq!(mood, Some(-3)),
        _ => panic!("expected checkin command"),
    }

    let cli = MonoCLI::try_parse_from(["mono", "todo", "text", "-i", "2"]).unwrap();
    match cli.command {
        Some(MonoCommands::Todo { priority, .. }) => assert_eq!(priority, Some(2)),
        _ => panic!("expected todo command"),
    }

    let cli = MonoCLI::try_parse_from(["mono", "idea", "text", "-y", "5"]).unwrap();
    match cli.command {
        Some(MonoCommands::Idea { hype, .. }) => assert_eq!(hype, Some(5)),
        _ => panic!("expected idea command"),
    }

    assert!(MonoCLI::try_parse_from(["mono", "checkin", "text", "-m", "6"]).is_err());
    assert!(MonoCLI::try_parse_from(["mono", "todo", "text", "-i", "6"]).is_err());
    assert!(MonoCLI::try_parse_from(["mono", "idea", "text", "-y", "6"]).is_err());
}

#[test]
fn stats_render_mood_views_for_scored_checkins() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let today = Local::now().date_naive();
    let checkin = |days_back: i64, value: Option<Score>| -> Result<(), MonoError> {
        db.add_row(
            "mood".to_string(),
            NoteTypes::CheckIn,
            PathBuf::from("notes/mood.md"),
            (today - Duration::days(days_back))
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            value,
        )?;
        Ok(())
    };
    checkin(0, Some(Score::new(2)?))?;
    checkin(0, Some(Score::new(3)?))?;
    checkin(1, Some(Score::new(-1)?))?;
    checkin(4, None)?;
    checkin(15, Some(Score::new(1)?))?;

    let full = get_stats(&db, false, false)?;
    println!("{full:?}");
    assert!(full.contains("Mood (last 14 days)"));
    let today_row = full.lines().find(|line| line.contains("[+2, +3]")).unwrap();
    assert!(today_row.contains("+2.5"));

    let oldest_row = full
        .lines()
        .find(|line| line.contains("no check-in"))
        .unwrap();
    assert!(oldest_row.contains("+1.0"));

    assert!(full.contains("unscored"));
    assert!(full.contains("no check-in"));
    assert!(full.contains("Logged: 3/4 scored"));

    let short = get_stats(&db, true, false)?;
    assert!(short.contains("Mood (14d)"));
    assert!(short.contains("7d avg"));
    assert!(!short.contains("Mood (last 14 days)"));

    Ok(())
}

#[test]
fn done_checks_off_todos_with_a_positive_score() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let add = |title: &str, kind: NoteTypes, value: Option<Score>| -> Result<u32, MonoError> {
        let path = directory.path().join(format!("{title}.md"));
        fs::write(&path, title)?;
        Ok(db.add_row(
            title.to_string(),
            kind,
            path,
            today.and_hms_opt(9, 30, 0).unwrap(),
            value,
        )?)
    };

    let open = add("open", NoteTypes::Todo, Some(Score::new(4)?))?;
    let checked = add("checked", NoteTypes::Todo, Some(Score::new(-2)?))?;
    let unscored = add("unscored", NoteTypes::Todo, None)?;
    let idea = add("idea", NoteTypes::Other, Some(Score::new(1)?))?;

    let output = handle_todos(&db, vec![open, checked, unscored, idea], true)?;
    let expected = [
        format!("Checked off note {open}."),
        format!("Note {checked} is already checked off."),
        format!("Note {unscored} has no value to check off."),
        format!("Note {idea} is a other, not a todo. Only todo's can be checked on/off."),
    ];
    assert_eq!(output.len(), expected.len());
    for line in expected {
        assert!(output.contains(&line), "missing {line:?} in {output:?}");
    }

    assert_eq!(db.read_single_row(open)?.value, Some(Score::new(-4)?));
    assert_eq!(db.read_single_row(checked)?.value, Some(Score::new(-2)?));
    assert_eq!(db.read_single_row(unscored)?.value, None);

    assert_eq!(
        handle_todos(&db, vec![open], true)?,
        [format!("Note {open} is already checked off.")]
    );
    assert_eq!(db.read_single_row(open)?.value, Some(Score::new(-4)?));

    Ok(())
}

#[test]
fn done_reports_missing_note_ids_and_still_handles_the_rest() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("real.md");
    fs::write(&path, "real")?;
    let real = db.add_row(
        "real".to_string(),
        NoteTypes::Todo,
        path,
        Local::now().date_naive().and_hms_opt(9, 30, 0).unwrap(),
        Some(Score::new(3)?),
    )?;
    let missing = real + 999;

    let output = handle_todos(&db, vec![real, missing], true)?;

    assert_eq!(output.len(), 2);
    assert!(output.contains(&format!("Checked off note {real}.")));
    assert!(output.contains(&format!("Note {missing} does not exist.")));
    assert_eq!(db.read_single_row(real)?.value, Some(Score::new(-3)?));

    Ok(())
}
