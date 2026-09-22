pub enum CommandOutput {
    Text(String),
    Lines(Vec<String>),
}

impl CommandOutput {
    pub fn as_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Lines(lines) => lines.join("\n"),
        }
    }

    pub fn print(&self) {
        match self {
            Self::Text(text) => print!("{text}"),
            Self::Lines(lines) => {
                for line in lines {
                    println!("{line}");
                }
            }
        }
    }
}
