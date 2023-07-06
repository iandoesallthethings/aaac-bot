use chrono::prelude::*;
mod ical;
use ical::*;

// Deploying to vercel
// https://github.com/vercel-community/rust

fn main() {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let url = format!("https://flagpole.com/?post_type=tribe_events&ical=1&tribe-bar-date={date}");

    let events: Vec<_> = get_events(url)
        .into_iter()
        .filter(|event| event.categories.contains(&"ART".to_string()))
        .collect();

    dbg!(&events);
}
