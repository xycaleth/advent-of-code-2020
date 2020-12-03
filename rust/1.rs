use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn solve(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    for i in numbers.iter() {
        for j in numbers.iter().rev() {
            let s = i + j;
            if s < target {
                break;
            }

            if s == target {
                return Some(i * j);
            }
        }
    }

    return None;
}

fn part1(numbers: &Vec<i32>, target: i32) {
    match solve(&numbers, target) {
        Some(result) => println!("{}", result),
        None => println!("No solution"),
    };
}

fn part2(numbers: &Vec<i32>, target: i32) {
    for (i, number) in numbers.iter().enumerate() {
        let new_target = target - number;
        match solve(&numbers[i + 1..].to_vec(), new_target) {
            Some(result) => println!("{}", number * result),
            None => (),
        };
    }
}

fn main() {
    let path = Path::new("../input1.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let numbers = {
        let numbers_iter = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap());
        let mut numbers = numbers_iter.collect::<Vec<i32>>();
        numbers.sort();
        numbers
    };

    let target = 2020;
    part1(&numbers, target);
    part2(&numbers, target);
}
