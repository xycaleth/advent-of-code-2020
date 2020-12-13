fn main() {
    let input = include_str!("../input13.txt");
    let mut lines = input.lines();

    let earliest_time: u64 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<(usize, u64)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(index, bus)| (index, bus.parse::<u64>()))
        .filter(|(_, bus)| bus.is_ok())
        .map(|(index, bus)| (index, bus.unwrap()))
        .collect();

    fn next_departure_after(t: u64, freq: u64) -> u64 {
        freq * ((t + freq - 1) / freq)
    }

    // part 1
    let (bus, best_time) = buses
        .iter()
        .map(|(_, n)| (n, next_departure_after(earliest_time, *n)))
        .filter(|(_, depart_time)| *depart_time >= earliest_time)
        .min_by_key(|(_, depart_time)| *depart_time)
        .unwrap();

    println!("Best bus is {}, departing at {}", bus, best_time);
    println!("{}", bus * (best_time - earliest_time));

    // part 2
    let (least_freq_index, least_freq_bus) = buses.iter().max_by_key(|(_, bus)| *bus).unwrap();

    let y = (next_departure_after(100000000000000, *least_freq_bus)..)
        .step_by(*least_freq_bus as usize)
        .find(|depart_time| {
            buses.iter().all(|(bus_index, bus)| {
                let expected_depart_time =
                    (*depart_time as usize - *least_freq_index) + bus_index;
                (expected_depart_time % *bus as usize) == 0
            })
        });

    match y {
        Some(num) => println!("t = {}", num - least_freq_index),
        None => println!("No solution"),
    }
}
