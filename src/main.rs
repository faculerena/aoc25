#![allow(dead_code)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day2::full("src/day2/input")?;
    Ok(())
}

mod day1;
mod day2;
mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;