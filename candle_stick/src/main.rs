use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration};
use plotters::prelude::*;

fn parse_time(t: &str) -> Date<Local>{
    Local
        .datetime_from_str(&format!("{} 0:0", t), %Y-%m-%d %H:%M)
        .unwrap()
        .date()
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = get_data();
    let root = BitMapBackend::new("plotters-doc-data/stock.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (to_date, from_date) = (
        parse_time
    )
}