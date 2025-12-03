#![allow(dead_code)]
mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day2::full("src/day2/input")?;
    Ok(())
}
