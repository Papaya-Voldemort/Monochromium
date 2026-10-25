use crate::types::Note;
use crate::utils::day_of;
use chrono::{Datelike, Duration, NaiveDate};
use owo_colors::{OwoColorize, Stream};
use std::collections::HashMap;

pub const MOOD_SHOWN: usize = 14;
pub const MOOD_SPAN: usize = 20;
const ROLLING_WINDOW: usize = 7;
pub struct MoodDay {
    pub date: NaiveDate,
    pub scores: Vec<Option<i32>>,
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

fn paint(text: &str, style: impl Fn(&str) -> String) -> String {
    text.if_supports_color(Stream::Stdout, |t| style(t))
        .to_string()
}

fn dim(text: &str) -> String {
    paint(text, |t| t.dimmed().to_string())
}

fn bold(text: &str) -> String {
    paint(text, |t| t.bold().to_string())
}

fn green(text: &str) -> String {
    paint(text, |t| t.green().to_string())
}

fn bright_green(text: &str) -> String {
    paint(text, |t| t.bright_green().to_string())
}

fn red(text: &str) -> String {
    paint(text, |t| t.red().to_string())
}

fn bright_red(text: &str) -> String {
    paint(text, |t| t.bright_red().to_string())
}

pub fn mood_table(days: &[MoodDay]) -> String {
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
            mood_bar(daily[i]),
            daily_cell(daily[i]),
            score_pills(&day.scores),
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
            dim("Peak:"),
            date.format("%b %d"),
            green(&signed(avg))
        ));
    }
    if let Some((date, avg)) = worst {
        parts.push(format!(
            "{} {} ({})",
            dim("Low:"),
            date.format("%b %d"),
            red(&signed(avg))
        ));
    }
    parts.push(format!("{} {scored}/{total} scored", dim("Logged:")));

    lines.push(String::new());
    lines.push(format!("  {}", parts.join("   ")));
    lines.join("\n")
}

fn mood_bar(avg: Option<f64>) -> String {
    let Some(avg) = avg else {
        return format!("     {}     ", dim("┃"));
    };
    if avg == 0.0 {
        return format!("     {}     ", bold("╋"));
    }
    let mut blocks = "█".repeat(avg.abs() as usize);
    if avg.abs().fract() >= 0.5 {
        blocks.push(if avg > 0.0 { '▌' } else { '▐' });
    }
    if avg > 0.0 {
        format!("     ┃{}", green(&format!("{blocks:<5}")))
    } else {
        format!("{}┃     ", red(&format!("{blocks:>5}")))
    }
}

fn daily_cell(avg: Option<f64>) -> String {
    let Some(avg) = avg else {
        return format!("    {}", dim("—"));
    };
    let plain = signed(avg);
    let pad = " ".repeat(5 - plain.len());
    let colored = if avg > 0.0 {
        green(&plain)
    } else if avg < 0.0 {
        red(&plain)
    } else {
        plain
    };
    format!("{pad}{colored}")
}

fn score_pills(scores: &[Option<i32>]) -> String {
    if scores.is_empty() {
        return dim("no check-in");
    }
    let pills: Vec<String> = scores
        .iter()
        .map(|score| match score {
            None => dim("unscored"),
            Some(0) => "0".to_string(),
            Some(v) if *v > 0 => green(&format!("{v:+}")),
            Some(v) => red(&format!("{v:+}")),
        })
        .collect();
    format!("[{}]", pills.join(", "))
}

pub fn mood_footer(days: &[MoodDay]) -> String {
    let first = days.len() - MOOD_SHOWN;
    let shown = &days[first..];
    let week = MOOD_SHOWN / 2;

    let glyphs: Vec<String> = shown.iter().map(|d| mood_glyph(&d.scores)).collect();
    let daily: Vec<Option<f64>> = days.iter().map(|d| daily_avg(&d.scores)).collect();
    let last = rolling_avg(&daily, daily.len() - 1);
    let avg = last.map_or_else(|| dim("·"), |v| bold(&signed(v)));

    // "Oct 11–17" within a month, "Sep 28–Oct 4" across the boundary
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
            format!("    range {}..{}", dim(&signed(*w)), dim(&signed(*b)))
        }
        _ => String::new(),
    };

    format!(
        "  {:<15}[{} {} {}]  7d avg {avg}\n{}{}   {}{}",
        format!("Mood ({MOOD_SHOWN}d)"),
        glyphs[..week].join(" "),
        dim("│"),
        glyphs[week..].join(" "),
        " ".repeat(18),
        dim(&format!("{:<13}", label(0, week - 1))),
        dim(&format!("{:<13}", label(week, MOOD_SHOWN - 1))),
        range,
    )
}

fn mood_glyph(scores: &[Option<i32>]) -> String {
    if scores.is_empty() {
        return dim("·");
    }
    match daily_avg(scores) {
        None => dim("○"),
        Some(0.0) => "▬".to_string(),
        Some(avg) if avg >= 3.0 => bright_green("▲"),
        Some(avg) if avg > 0.0 => green("▴"),
        Some(avg) if avg <= -3.0 => bright_red("▼"),
        Some(_) => red("▾"),
    }
}
