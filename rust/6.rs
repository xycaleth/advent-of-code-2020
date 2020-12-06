use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input6.txt");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut yeses = Vec::new();
    let mut answered = Vec::new();

    for line in reader.lines().filter_map(Result::ok) {
        if line.is_empty() {
            answered.sort();
            answered.dedup();
            yeses.push(answered.len());

            answered.clear();
        } else {
            answered.extend(line.chars());
            println!("{}", line);
        }
    }

    answered.sort();
    answered.dedup();
    yeses.push(answered.len());

    answered.clear();

    println!("{}", yeses.iter().sum::<usize>());

    Ok(())
}
