use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn calculate_pairs(numbers: &Vec<i64>, skip: usize, window_size: usize) -> Vec<i64> {
    let mut pairs = Vec::new();
    for (i, num) in numbers.iter().skip(skip).take(window_size).enumerate() {
        for j in (i + 1)..window_size {
            pairs.push(num + numbers[skip + j]);
        }
    }
    pairs
}

fn find_invalid_number(numbers: &Vec<i64>) -> Option<i64> {
    const WINDOW_SIZE: usize = 25;

    let mut pairs = calculate_pairs(&numbers, 0, WINDOW_SIZE);

    for (i, num) in numbers.iter().skip(WINDOW_SIZE).enumerate() {
        if !pairs.iter().any(|n| n == num) {
            return Some(*num);
        }

        pairs = calculate_pairs(&numbers, i + 1, WINDOW_SIZE);
    }

    None
}

fn find_weakness(numbers: &Vec<i64>, num: i64) -> Option<i64> {
    if let Some(index) = numbers.iter().position(|&n| n == num) {
        for window_size in 2..index / 2 {
            for window in numbers.windows(window_size) {
                let total = window.iter().sum::<i64>();
                if total == num {
                    let min = window.iter().min().unwrap();
                    let max = window.iter().max().unwrap();
                    return Some(min + max);
                }
            }
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input9.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(Result::ok);
    let numbers = lines
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    match find_invalid_number(&numbers) {
        Some(invalid_number) => {
            println!("{} is not valid", invalid_number);

            match find_weakness(&numbers, invalid_number) {
                Some(weakness) => println!("{} is the weakness!", weakness),
                None => println!("There is no weakness!"),
            }
        }

        None => panic!("No invalid number!"),
    }

    Ok(())
}
