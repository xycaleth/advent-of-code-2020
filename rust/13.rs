use std::cmp::Reverse;

fn main() {
    let input = include_str!("../input13.txt");
    let mut lines = input.lines();

    let earliest_time: u64 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(index, bus)| (index, bus.parse::<u64>()))
        .filter(|(_, bus)| bus.is_ok())
        .map(|(index, bus)| (index as u64, bus.unwrap()))
        .collect();

    // part 1
    let (bus, best_time) = buses
        .iter()
        .map(|(_, n)| (n, n * ((earliest_time + n - 1) / n)))
        .filter(|(_, depart_time)| *depart_time >= earliest_time)
        .min_by_key(|(_, depart_time)| *depart_time)
        .unwrap();

    println!("Best bus is {}, departing at {}", bus, best_time);
    println!("{}", bus * (best_time - earliest_time));

    // part 2
    let mut step = 1;
    let mut t = 0;

    for (index, bus) in buses {
        t += step;

        while ((t + index) % bus) != 0 {
            t += step;
        }

        step *= bus;
    }

    println!("t = {}", t);
}
