use std::{error::Error, fs::read_to_string};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let zeros = file
        .lines()
        .try_fold((50i32, 0), |(pos, count), line| {
            let (dir, num_str) = line.split_at(1);
            let distance: i32 = num_str.parse()?;

            let new_pos = match dir.chars().next() {
                Some('R') => pos + distance,
                Some('L') => pos - distance,
                _ => unreachable!(),
            }
            .rem_euclid(100);

            Ok::<_, Box<dyn Error>>((new_pos, count + (new_pos == 0) as i32))
        })?
        .1;

    println!("p1: {}", zeros);
    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let zeros = file
        .lines()
        .try_fold((50i32, 0), |(pos, count), line| {
            let (dir, num_str) = line.split_at(1);
            let distance: i32 = num_str.parse()?;

            let new_pos_raw = match dir.chars().next() {
                Some('R') => pos + distance,
                Some('L') => pos - distance,
                _ => unreachable!(),
            };

            let crossings = if new_pos_raw >= pos {
                new_pos_raw.div_euclid(100)
            } else {
                let upper = if pos == 0 {
                    -1
                } else {
                    (pos - 1).div_euclid(100)
                };
                upper - (new_pos_raw - 1).div_euclid(100)
            };

            Ok::<_, Box<dyn Error>>((new_pos_raw.rem_euclid(100), count + crossings))
        })?
        .1;

    println!("p2: {}", zeros);
    Ok(())
}
