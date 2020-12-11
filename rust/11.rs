use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

struct Seats {
    state: Vec<Vec<Cell>>,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Cell::Floor => '.',
                Cell::EmptySeat => 'L',
                Cell::OccupiedSeat => '#',
            }
        )
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.state.iter() {
            for seat in row.iter() {
                write!(f, "{}", seat);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl Seats {
    fn new() -> Self {
        Seats { state: Vec::new() }
    }

    fn add_row(&mut self, row: &str) {
        self.state.push(
            row.chars()
                .map(|c| match c {
                    '.' => Cell::Floor,
                    'L' => Cell::EmptySeat,
                    '#' => Cell::OccupiedSeat,
                    _ => panic!("Invalid char '{}'", c),
                })
                .collect::<Vec<_>>(),
        );
    }

    fn step(
        &mut self,
        move_factor: i32,
        count_neighbors: &dyn Fn(&Self, usize, usize) -> i32,
    ) -> bool {
        let new_state = self
            .state
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(seat, c)| match c {
                        Cell::EmptySeat => {
                            if count_neighbors(self, row_index, seat) == 0 {
                                Cell::OccupiedSeat
                            } else {
                                *c
                            }
                        }
                        Cell::OccupiedSeat => {
                            if count_neighbors(self, row_index, seat) >= move_factor {
                                Cell::EmptySeat
                            } else {
                                *c
                            }
                        }
                        _ => *c,
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();

        let changed = self.state == new_state;
        self.state = new_state;
        return changed;
    }

    fn get_cell(&self, row: i32, seat: i32) -> Option<Cell> {
        if row < 0 || (row as usize) >= self.state.len() {
            None
        } else if seat < 0 || (seat as usize) >= self.state[0].len() {
            None
        } else {
            Some(self.state[row as usize][seat as usize])
        }
    }

    fn num_occupied(&self) -> usize {
        self.state
            .iter()
            .map(|row| row.iter().filter(|&c| *c == Cell::OccupiedSeat).count())
            .sum()
    }

    fn reset(&mut self) {
        for row_index in 0..self.state.len() {
            self.state[row_index] = self.state[row_index]
                .iter()
                .map(|c| match c {
                    Cell::OccupiedSeat => Cell::EmptySeat,
                    _ => *c,
                })
                .collect();
        }
    }
}

fn num_neighbors1(seats: &Seats, row: usize, seat: usize) -> i32 {
    let mut neighbours = 0;

    //println!("Checking neighbours for row {} seat {}", row, seat);
    for offset in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let x = (seat as i32) + offset.0;
        let y = (row as i32) + offset.1;

        if seats.get_cell(y, x) == Some(Cell::OccupiedSeat) {
            //println!("  row {} seat {} is a neighbour!", y, x);
            neighbours += 1;
        }
    }

    neighbours
}

fn num_neighbors2(seats: &Seats, row: usize, seat: usize) -> i32 {
    let mut neighbours = 0;

    //println!("Checking neighbours for row {} seat {}", row, seat);
    for offset in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let mut x = (seat as i32) + offset.0;
        let mut y = (row as i32) + offset.1;

        loop {
            match seats.get_cell(y, x) {
                None => break,
                Some(Cell::EmptySeat) => break,
                Some(Cell::OccupiedSeat) => {
                    neighbours += 1;
                    break;
                }
                _ => (),
            }

            x += offset.0;
            y += offset.1;
        }
    }

    neighbours
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("../input11.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(Result::ok);

    let mut seats = Seats::new();
    for line in lines {
        seats.add_row(&line);
    }

    while !seats.step(4, &num_neighbors1) {}
    println!("{}", seats.num_occupied());

    seats.reset();
    while !seats.step(5, &num_neighbors2) {}
    println!("{}", seats.num_occupied());

    Ok(())
}
