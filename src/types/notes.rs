use chrono::NaiveDateTime;
use clap::ValueEnum;
use std::fmt;

#[derive(ValueEnum, Clone, Debug)]
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

#[derive(Debug, PartialEq)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub _note_type: String,
    pub date: NaiveDateTime,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum EditMode {
    Append,
    Overwrite,
}
