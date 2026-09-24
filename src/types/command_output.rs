pub enum CommandOutput {
    Text(String),
    Lines(Vec<String>),
    Raw(String),
}

impl CommandOutput {
    pub fn as_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Lines(lines) => lines.join("\n"),
            Self::Raw(text) => text.clone(),
        }
    }

    pub fn print(&self) {
        match self {
            Self::Text(text) => println!("{text}"),
            Self::Lines(lines) => {
                if lines.is_empty() {
                    println!("No notes found :(")
                } else {
                    for line in lines {
                        println!("{line}");
                    }
                }
            }
            Self::Raw(text) => print!("{text}"),
        }
    }
}
