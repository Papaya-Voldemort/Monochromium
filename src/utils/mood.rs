use crate::types::Note;
use crate::utils::day_of;
use chrono::{Datelike, Duration, NaiveDate};
use owo_colors::OwoColorize;
use std::collections::HashMap;

pub const MOOD_SHOWN: usize = 14;
pub const MOOD_SPAN: usize = 20;
const ROLLING_WINDOW: usize = 7;
pub struct MoodDay {
    pub date: NaiveDate,
    pub scores: Vec<Option<i32>>,
}

pub struct Ink {
    colorize: bool,
}

impl Ink {
    fn paint(&self, text: &str, style: impl Fn(&str) -> String) -> String {
        if self.colorize {
            style(text)
        } else {
            text.to_string()
        }
    }

    pub fn dim(&self, text: &str) -> String {
        self.paint(text, |t| t.dimmed().to_string())
    }
    pub fn bold(&self, text: &str) -> String {
        self.paint(text, |t| t.bold().to_string())
    }
    pub fn green(&self, text: &str) -> String {
        self.paint(text, |t| t.green().to_string())
    }
    pub fn bright_green(&self, text: &str) -> String {
        self.paint(text, |t| t.bright_green().to_string())
    }
    pub fn red(&self, text: &str) -> String {
        self.paint(text, |t| t.red().to_string())
    }
    pub fn bright_red(&self, text: &str) -> String {
        self.paint(text, |t| t.bright_red().to_string())
    }
}

pub fn fetch_mood_days(today: NaiveDate, notes: Vec<Note>) -> Vec<MoodDay> {
    let mut by_day: HashMap<NaiveDate, Vec<Option<i32>>> = HashMap::new();
    for note in notes {
        by_day.entry(day_of(&note)).or_default().push(note.value);
    }

    (0..MOOD_SPAN as i64)
        .rev()
        .map(|back| {
            let date = today - Duration::days(back);
            MoodDay {
                scores: by_day.remove(&date).unwrap_or_default(),
                date,
            }
        })
        .collect()
}

fn mean(vals: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, count) = vals.fold((0.0, 0usize), |(sum, n), v| (sum + v, n + 1));
    (count > 0).then(|| sum / count as f64)
}

fn daily_avg(scores: &[Option<i32>]) -> Option<f64> {
    mean(scores.iter().flatten().map(|&v| v as f64))
}

fn rolling_avg(daily: &[Option<f64>], end: usize) -> Option<f64> {
    let start = end.saturating_sub(ROLLING_WINDOW - 1);
    mean(daily[start..=end].iter().filter_map(|&v| v))
}
type Extreme = Option<(NaiveDate, f64)>;

fn best_and_worst(shown: &[MoodDay]) -> (Extreme, Extreme) {
    let mut best = None;
    let mut worst = None;
    for day in shown {
        let Some(avg) = daily_avg(&day.scores) else {
            continue;
        };
        if best.is_none_or(|(_, b)| avg > b) {
            best = Some((day.date, avg));
        }
        if worst.is_none_or(|(_, w)| avg < w) {
            worst = Some((day.date, avg));
        }
    }
    (best, worst)
}

fn signed(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:+}", value as i64)
    } else {
        format!("{value:+.1}")
    }
}
pub fn mood_table(days: &[MoodDay], colorize: bool) -> String {
    let ink = Ink { colorize };
    let first = days.len() - MOOD_SHOWN;
    let daily: Vec<Option<f64>> = days.iter().map(|d| daily_avg(&d.scores)).collect();
    let rolling: Vec<Option<f64>> = (0..daily.len()).map(|i| rolling_avg(&daily, i)).collect();
    let shown = &days[first..];

    let mut lines = vec![
        format!("Mood (last {MOOD_SHOWN} days)"),
        String::new(),
        format!(
            "  {:<6}  {:^11}  {:>5}  {:>6}  Check-ins",
            "Day", "Mood", "Daily", "7d Avg"
        ),
    ];

    for (row, day) in shown.iter().enumerate() {
        let i = first + row;
        let roll = rolling[i].map_or_else(|| "·".to_string(), |v| format!("{v:+.1}"));
        lines.push(format!(
            "  {}  {}  {}  {roll:>6}  {}",
            day.date.format("%b %d"),
            mood_bar(daily[i], colorize),
            daily_cell(daily[i], colorize),
            score_pills(&day.scores, colorize),
        ));
    }

    let (best, worst) = best_and_worst(shown);
    let total: usize = shown.iter().map(|d| d.scores.len()).sum();
    let scored: usize = shown
        .iter()
        .map(|d| d.scores.iter().flatten().count())
        .sum();

    let mut parts = Vec::new();
    if let Some((date, avg)) = best {
        parts.push(format!(
            "{} {} ({})",
            ink.dim("Peak:"),
            date.format("%b %d"),
            ink.green(&signed(avg))
        ));
    }
    if let Some((date, avg)) = worst {
        parts.push(format!(
            "{} {} ({})",
            ink.dim("Low:"),
            date.format("%b %d"),
            ink.red(&signed(avg))
        ));
    }
    parts.push(format!("{} {scored}/{total} scored", ink.dim("Logged:")));

    lines.push(String::new());
    lines.push(format!("  {}", parts.join("   ")));
    lines.join("\n")
}

fn mood_bar(avg: Option<f64>, colorize: bool) -> String {
    let ink = Ink { colorize };
    let Some(avg) = avg else {
        return format!("     {}     ", ink.dim("┃"));
    };
    if avg == 0.0 {
        return format!("     {}     ", ink.bold("╋"));
    }
    let mut blocks = "█".repeat(avg.abs() as usize);
    if avg.abs().fract() >= 0.5 {
        blocks.push(if avg > 0.0 { '▌' } else { '▐' });
    }
    if avg > 0.0 {
        format!("     ┃{}", ink.green(&format!("{blocks:<5}")))
    } else {
        format!("{}┃     ", ink.red(&format!("{blocks:>5}")))
    }
}

fn daily_cell(avg: Option<f64>, colorize: bool) -> String {
    let ink = Ink { colorize };
    let Some(avg) = avg else {
        return format!("    {}", ink.dim("—"));
    };
    let plain = signed(avg);
    let pad = " ".repeat(5 - plain.len());
    let colored = if avg > 0.0 {
        ink.green(&plain)
    } else if avg < 0.0 {
        ink.red(&plain)
    } else {
        plain
    };
    format!("{pad}{colored}")
}

fn score_pills(scores: &[Option<i32>], colorize: bool) -> String {
    let ink = Ink { colorize };
    if scores.is_empty() {
        return ink.dim("no check-in");
    }
    let pills: Vec<String> = scores
        .iter()
        .map(|score| match score {
            None => ink.dim("unscored"),
            Some(0) => "0".to_string(),
            Some(v) if *v > 0 => ink.green(&format!("{v:+}")),
            Some(v) => ink.red(&format!("{v:+}")),
        })
        .collect();
    format!("[{}]", pills.join(", "))
}

pub fn mood_footer(days: &[MoodDay], colorize: bool) -> String {
    let ink = Ink { colorize };
    let first = days.len() - MOOD_SHOWN;
    let shown = &days[first..];
    let week = MOOD_SHOWN / 2;

    let glyphs: Vec<String> = shown
        .iter()
        .map(|d| mood_glyph(&d.scores, colorize))
        .collect();
    let daily: Vec<Option<f64>> = days.iter().map(|d| daily_avg(&d.scores)).collect();
    let last = rolling_avg(&daily, daily.len() - 1);
    let avg = last.map_or_else(|| ink.dim("·"), |v| ink.bold(&signed(v)));

    let label = |from: usize, to: usize| {
        let start = shown[from].date;
        let end = shown[to].date;
        let end_fmt = if start.month() == end.month() {
            "%-d"
        } else {
            "%b %-d"
        };
        format!("{}–{}", start.format("%b %-d"), end.format(end_fmt))
    };

    let (best, worst) = best_and_worst(shown);
    let range = match (&best, &worst) {
        (Some((_, b)), Some((_, w))) => {
            format!(
                "    range {}..{}",
                ink.dim(&signed(*w)),
                ink.dim(&signed(*b))
            )
        }
        _ => String::new(),
    };

    format!(
        "  {:<15}[{} {} {}]  7d avg {avg}\n{}{}   {}{}",
        format!("Mood ({MOOD_SHOWN}d)"),
        glyphs[..week].join(" "),
        ink.dim("│"),
        glyphs[week..].join(" "),
        " ".repeat(18),
        ink.dim(&format!("{:<13}", label(0, week - 1))),
        ink.dim(&format!("{:<13}", label(week, MOOD_SHOWN - 1))),
        range,
    )
}

fn mood_glyph(scores: &[Option<i32>], colorize: bool) -> String {
    let ink = Ink { colorize };
    if scores.is_empty() {
        return ink.dim("·");
    }
    match daily_avg(scores) {
        None => ink.dim("○"),
        Some(0.0) => "▬".to_string(),
        Some(avg) if avg >= 3.0 => ink.bright_green("▲"),
        Some(avg) if avg > 0.0 => ink.green("▴"),
        Some(avg) if avg <= -3.0 => ink.bright_red("▼"),
        Some(_) => ink.red("▾"),
    }
}
