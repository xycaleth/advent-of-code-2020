use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Grid {
    rows: Vec<Vec<bool>>,
}

impl Grid {
    fn new(rows: Vec<Vec<bool>>) -> Grid {
        return Grid { rows: rows };
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        let row = &self.rows[y];
        return row[x % row.len()];
    }

    fn count_trees(&self, move_x: usize, move_y: usize) -> i32 {
        let mut count = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        while y < self.rows.len() {
            if self.is_tree(x, y) {
                count += 1;
            }

            x += move_x;
            y += move_y;
        }

        return count;
    }
}

fn part1(grid: &Grid) {
    println!("{}", grid.count_trees(3, 1));
}

fn part2(grid: &Grid) {
    let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result: i64 = 1;
    for (move_x, move_y) in moves {
        result *= grid.count_trees(move_x, move_y) as i64;
    }

    println!("{}", result);
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input3.txt");

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let grid = Grid::new(
        lines
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|x| x == '#')
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>(),
    );

    part1(&grid);
    part2(&grid);

    Ok(())
}
