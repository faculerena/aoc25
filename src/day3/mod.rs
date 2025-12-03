use std::{error::Error, fs::read_to_string};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let k = file
        .lines()
        .fold(0, |acc, line| acc + find_joltage_size(line, 2));

    println!("{k}");
    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let k = file
        .lines()
        .fold(0u128, |acc, line| acc + find_joltage_size(line, 12));
    println!("{k}");
    Ok(())
}

fn find_joltage_size(bank: &str, size: usize) -> u128 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();
    let n = digits.len();

    digits
        .iter()
        .enumerate()
        .fold(
            (Vec::with_capacity(12), 0),
            |(mut stack, mut removed), (i, &digit)| {
                while !stack.is_empty()
                    && stack.last().unwrap() < &digit
                    && removed < (n - size)
                    && stack.len() + n - i > size
                {
                    stack.pop();
                    removed += 1;
                }

                if stack.len() < size {
                    stack.push(digit);
                } else {
                    removed += 1;
                }

                (stack, removed)
            },
        )
        .0
        .into_iter()
        .fold(0u128, |acc, digit| acc * 10 + digit as u128)
}

fn _find_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

    let suffix_maxes = digits
        .iter()
        .rev()
        .scan(0, |max, &d| {
            *max = (*max).max(d);
            Some(*max)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    digits
        .iter()
        .zip(suffix_maxes.skip(1))
        .fold(0, |best, (&first, second_best)| {
            best.max(first * 10 + second_best)
        })
}
