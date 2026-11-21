use crate::types::{MonoError, Note};
use chrono::NaiveDateTime;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use std::fmt;
use std::ops::RangeInclusive;

pub const SCORE_MIN: i8 = -5;
pub const SCORE_MAX: i8 = 5;

/// The only range a score may hold, shared by the CLI, the parser and SQLite.
pub const SCORE_RANGE: RangeInclusive<i64> = SCORE_MIN as i64..=SCORE_MAX as i64;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score(i8);

impl Score {
    pub fn new(value: i64) -> Result<Self, MonoError> {
        SCORE_RANGE
            .contains(&value)
            .then_some(Self(value as i8))
            .ok_or_else(Self::invalid)
    }

    pub fn parse(s: &str) -> Result<Self, MonoError> {
        let value: i64 = s.trim().parse().map_err(|_| Self::invalid())?;
        Self::new(value)
    }

    pub const fn get(self) -> i8 {
        self.0
    }

    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn flip(self) -> Self {
        Self(-self.0)
    }

    fn invalid() -> MonoError {
        MonoError::InvalidInput(format!(
            "Values must be a valid number from {SCORE_MIN} to +{SCORE_MAX}."
        ))
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSql for Score {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.0.into())
    }
}

impl FromSql for Score {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let raw = i64::column_result(value)?;
        SCORE_RANGE
            .contains(&raw)
            .then_some(Self(raw as i8))
            .ok_or(FromSqlError::OutOfRange(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_accepts_only_the_documented_range() {
        for value in SCORE_MIN as i64..=SCORE_MAX as i64 {
            assert_eq!(Score::new(value).unwrap().get() as i64, value);
            assert_eq!(
                Score::parse(&value.to_string()).unwrap().get() as i64,
                value
            );
        }

        for value in [-6, 6, i64::MIN, i64::MAX] {
            assert!(Score::new(value).is_err());
            assert!(Score::parse(&value.to_string()).is_err());
        }

        assert!(Score::parse("not a number").is_err());
    }

    #[test]
    fn score_flips_without_changing_width() {
        assert_eq!(Score::new(3).unwrap().flip(), Score::new(-3).unwrap());
        assert_eq!(Score::new(-3).unwrap().flip(), Score::new(3).unwrap());
        assert_eq!(Score::new(0).unwrap().flip(), Score::new(0).unwrap());
        assert_eq!(Score::new(-5).unwrap().flip(), Score::new(5).unwrap());
    }

    #[test]
    fn score_round_trips_through_sql() -> Result<(), MonoError> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.execute_batch("CREATE TABLE scores (value INTEGER)")?;

        for value in [SCORE_MIN, 0, SCORE_MAX] {
            let score = Score::new(value as i64)?;
            let stored: Option<Score> =
                conn.query_row("SELECT ?1", rusqlite::params![score], |row| row.get(0))?;
            assert_eq!(stored, Some(score));
        }

        let out_of_range: Result<Score, _> =
            conn.query_row("SELECT ?1", rusqlite::params![9], |row| row.get(0));
        assert!(out_of_range.is_err());

        Ok(())
    }
}
