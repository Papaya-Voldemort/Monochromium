use crate::types::MonoError;
use chrono::NaiveDateTime;
use clap::ValueEnum;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum NoteTypes {
    Idea,
    #[value(name = "check-in", alias = "checkin")]
    CheckIn,
    Todo,
    Other,
}

impl NoteTypes {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Idea => "idea",
            Self::CheckIn => "check in",
            Self::Todo => "todo",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for NoteTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for NoteTypes {
    type Err = MonoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idea" => Ok(Self::Idea),
            "check in" => Ok(Self::CheckIn),
            "todo" => Ok(Self::Todo),
            "other" => Ok(Self::Other),
            _ => Err(MonoError::InvalidNoteType(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OldNote {
    pub id: u32,
    pub title: String,
    pub note_type: String,
    pub date: NaiveDateTime,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub note_type: NoteTypes,
    pub date: NaiveDateTime,
    pub file_link: PathBuf,
    pub content: Option<String>,
}

// TODO: DB no longer controls content so this needs tweaking
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EditMode {
    Title,
    FilePath,
    Type,
    Date,
}
