use std::{error::Error, fs::read_to_string};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let _file = read_to_string(file)?;

    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let _file = read_to_string(file)?;

    Ok(())
}
