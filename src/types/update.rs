use crate::types::{MonoError, Note};
use chrono::NaiveDateTime;
use rusqlite::types::ToSqlOutput;

pub enum FieldUpdate {
    Title(String),
    FilePath(String),
    Date(NaiveDateTime),
    NoteType(String),
    Value(Score),
}

impl FieldUpdate {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Title(_) => "title",
            Self::FilePath(_) => "file_link",
            Self::Date(_) => "date",
            Self::NoteType(_) => "type",
            Self::Value(_) => "value",
        }
    }

    pub fn sql_value(&self) -> ToSqlOutput<'_> {
        match self {
            Self::Title(v) | Self::FilePath(v) | Self::NoteType(v) => v.as_str().into(),
            Self::Date(d) => d.to_string().into(),
            Self::Value(s) => s.0.into(),
        }
    }
}

pub struct UpdateResult {
    pub change: FieldUpdate,
    pub before: Note,
    pub after: Note,
}

pub struct Score(i8);

impl Score {
    pub fn parse(s: &str) -> Result<Self, MonoError> {
        let n: i64 = s
            .trim()
            .parse()
            .map_err(|_| MonoError::InvalidInput("idk".to_string()))?;
        (-5..=5)
            .contains(&n)
            .then_some(Score(n as i8))
            .ok_or_else(|| {
                MonoError::InvalidInput("Values must be a valid number from -5 to +5.".to_string())
            })
    }
}
