use std::{error::Error, fs::read_to_string};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let _ = read_to_string(file)?;

    println!();
    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let _ = read_to_string(file)?;
    println!("");
    Ok(())
}
