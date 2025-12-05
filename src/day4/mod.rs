use std::{error::Error, fs::read_to_string, u8};

pub fn full(file: &str) -> Result<(), Box<dyn Error>> {
    p1(file)?;
    p2(file)?;
    Ok(())
}

pub fn p1(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let mut matrix: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;

    for col in 0..cols {
        for row in 0..rows {
            if matrix[row][col] == '@' {
                let around = diag_count(&mut matrix, row, col, rows, cols);
                if around < 4 {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");
    Ok(())
}

pub fn p2(file: &str) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(file)?;

    let mut matrix: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    let mut count = 0;
    let mut count_matrix: Vec<Vec<u8>> = vec![vec![u8::MAX; cols + 2]; rows + 2];
    let mut extractables = Vec::new();

    for col in 0..cols {
        for row in 0..rows {
            if matrix[row][col] == '@' {
                let around = diag_count(&mut matrix, row, col, rows, cols);
                count_matrix[row + 1][col + 1] = around as u8;
                if around < 4 {
                    extractables.push(sum((row as isize, col as isize), (1, 1)));
                }
            }
        }
    }

    while let Some(ext) = extractables.pop() {
        count += 1;

        for d in DIAGONALS {
            let ext_shifted = sum(ext, d);
            if count_matrix[ext_shifted.0 as usize][ext_shifted.1 as usize] == 4 {
                extractables.push(ext_shifted);
            }
            count_matrix[ext_shifted.0 as usize][ext_shifted.1 as usize] -= 1;
        }
    }

    println!("{count}");
    Ok(())
}

fn diag_count(
    matrix: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> i32 {
    let mut around = 0;

    for (dr, dc) in DIAGONALS.iter() {
        let new_pos = sum((row as isize, col as isize), (*dr, *dc));
        if new_pos.0 >= 0
            && new_pos.0 < rows as isize
            && new_pos.1 >= 0
            && new_pos.1 < cols as isize
        {
            if matrix[new_pos.0 as usize][new_pos.1 as usize] == '@' {
                around += 1;
            }
        }
    }

    around
}

const DIAGONALS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn sum(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}
