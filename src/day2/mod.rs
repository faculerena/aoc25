use std::{error::Error, fs::read_to_string};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

const PARSE: fn(&str) -> i128 = |s: &str| s.parse::<i128>().unwrap();

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let k = file.split(",").fold(0, |mut x, s| {
        let (a, b) = s
            .split_once("-")
            .map(|(a, b)| (PARSE(a), PARSE(b)))
            .unwrap();

        x += (a..=b)
            .filter(|i| {
                let s = i.to_string();
                let ls = s.len();
                let (p1, p2) = s.split_at(ls / 2);
                p1 == p2
            })
            .sum::<i128>();

        x
    });
    println!("{k}");
    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let k = file.split(",").fold(0, |mut x, s| {
        let (a, b) = s
            .split_once("-")
            .map(|(a, b)| (PARSE(a), PARSE(b)))
            .unwrap();

        x += (a..=b)
            .filter(|i| {
                let s = i.to_string();
                let ls = s.len();

                (1..ls).any(|n| ls % n == 0 && s == s[..n].repeat(ls / n))
            })
            .sum::<i128>();
        x
    });
    println!("{k}");
    Ok(())
}
