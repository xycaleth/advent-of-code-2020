use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn get_seat_id(encoded_seat: &str) -> i32 {
    let mut max_row = 127;
    let mut min_row = 0;

    let mut max_col = 7;
    let mut min_col = 0;
    for c in encoded_seat.chars() {
        match c {
            'F' => max_row = min_row + (max_row - min_row) / 2,
            'B' => min_row = min_row + (max_row - min_row + 1) / 2,
            'L' => max_col = min_col + (max_col - min_col) / 2,
            'R' => min_col = min_col + (max_col - min_col + 1) / 2,
            _ => ()
        }
    }

    return min_row * 8 + min_col;
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input5.txt");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let seat_ids = lines.map(|line| get_seat_id(&line.unwrap())).collect::<Vec<i32>>();
    println!("{}", seat_ids.iter().max().unwrap());

    let my_seat_id = {
        let mut sorted_seat_ids = seat_ids.clone();
        sorted_seat_ids.sort();

        let mut iter = sorted_seat_ids.iter();

        let mut last_seat_id = iter.next().unwrap();
        let mut expected_id = 0;

        for id in iter {
            expected_id = last_seat_id + 1;
            if id != &expected_id {
                break;
            }

            last_seat_id = id;
        }

        expected_id
    };

    println!("{}", my_seat_id);

    Ok(())
}
