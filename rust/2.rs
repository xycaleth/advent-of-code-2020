use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Entry {
    num1: i32,
    num2: i32,
    letter: char,
    password: String,
}

fn part1(entries: &Vec<Entry>) {
    let mut valid_count = 0;
    for entry in entries.iter() {
        let char_count = {
            let mut char_count = HashMap::new();
            for l in entry.password.chars() {
                *char_count.entry(l).or_insert(0) += 1;
            }

            char_count
        };

        match char_count.get(&entry.letter) {
            Some(count) => {
                if *count >= entry.num1 && *count <= entry.num2 {
                    valid_count += 1;
                }
            }
            _ => (),
        }
    }

    println!("{}", valid_count);
}

fn part2(entries: &Vec<Entry>) {
    let mut valid_count = 0;
    for entry in entries.iter() {
        let pw = &entry.password;
        let c1 = pw.chars().nth((entry.num1 - 1) as usize).unwrap();
        let c2 = pw.chars().nth((entry.num2 - 1) as usize).unwrap();

        if (c1 == entry.letter) ^ (c2 == entry.letter) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn main() {
    let path = Path::new("../input2.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let entries = {
        let mut entries = Vec::new();

        for line in lines {
            let s = line.unwrap();
            let pair = s.split(": ").collect::<Vec<&str>>();
            let rule = pair[0].split(" ").collect::<Vec<&str>>();
            let range = rule[0]
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            entries.push(Entry {
                num1: range[0],
                num2: range[1],
                letter: rule[1].parse::<char>().unwrap(),
                password: String::from(pair[1]),
            });
        }

        entries
    };

    part1(&entries);
    part2(&entries);
}
