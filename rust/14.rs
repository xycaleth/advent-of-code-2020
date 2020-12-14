use std::collections::HashMap;

struct Memory
{
    words: HashMap<usize, u64>,
}

impl Memory
{
    fn new() -> Self {
        Memory { words: HashMap::new() }
    }

    fn write(&mut self, address: usize, value: u64) {
        self.words.insert(address, value);
    }

    fn sum(&self) -> u64 {
        self.words.values().sum()
    }
}

fn write_v1(
    memory: &mut Memory,
    address: usize,
    value: u64,
    set_bits: u64,
    clear_bits: u64,
    _x_bits: u64,
) {
    memory.write(address, (value | set_bits) & clear_bits)
}

fn write_v2(
    memory: &mut Memory,
    address: usize,
    value: u64,
    set_bits: u64,
    clear_bits: u64,
    x_bits: u64,
) {
    let decoded_address: usize = ((address as u64) | set_bits) as usize;

    if x_bits == 0 {
        memory.write(decoded_address, value);
    } else {
        for bit_index in 0..36 {
            let bit = 1u64 << bit_index;
            if (x_bits & bit) != 0 {
                let changed_x_bits = x_bits & !bit;
                write_v2(
                    memory,
                    ((decoded_address as u64) | bit) as usize,
                    value,
                    set_bits,
                    clear_bits,
                    changed_x_bits,
                );
                write_v2(
                    memory,
                    ((decoded_address as u64) & !bit) as usize,
                    value,
                    set_bits,
                    clear_bits,
                    changed_x_bits,
                );
            }
        }
    }
}

fn run(input: &str, write: &dyn Fn(&mut Memory, usize, u64, u64, u64, u64)) -> u64 {
    let mut set_bits = 0u64;
    let mut clear_bits = 0xffffffffffffffffu64;
    let mut x_bits = 0u64;
    let mut memory = Memory::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let eq = line.find('=').unwrap();
            let mask = &line[(eq + 2)..];

            set_bits = 0u64;
            clear_bits = 0xffffffffffffffffu64;
            x_bits = 0u64;
            for (pos, c) in mask.chars().enumerate() {
                match c {
                    'X' => x_bits |= 1u64 << (35 - pos),
                    '1' => set_bits |= 1u64 << (35 - pos),
                    '0' => clear_bits &= !(1u64 << (35 - pos)),
                    _ => (),
                }
            }
        } else {
            let rbracket = line.find(']').unwrap();
            let address: usize = line[4..rbracket].parse().unwrap();

            let eq = line.find('=').unwrap();
            let num: u64 = line[(eq + 2)..].parse::<u64>().unwrap();

            //println!("line = {}", line);
            write(&mut memory, address, num, set_bits, clear_bits, x_bits);
        }
    }

    memory.sum()
}

fn main() {
    let input = include_str!("../input14.txt");

    let memory_sum1 = run(&input, &write_v1);
    let memory_sum2 = run(&input, &write_v2);

    println!("{}", memory_sum1);
    println!("{}", memory_sum2);
}
