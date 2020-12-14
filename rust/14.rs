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
    _clear_bits: u64,
    x_bits: u64,
) {
    let base_address: usize = ((address as u64) | set_bits) as usize;

    if x_bits == 0 {
        memory.write(base_address, value);
    } else {
        let floating_bits: Vec<u64> = (0..36).filter(|n| (x_bits & (1u64 << n)) != 0).collect();
        let num_floating = floating_bits.len();
        for i in 0..2usize.pow(num_floating as u32) {
            let mut this_address = base_address;
            for bit in 0..num_floating {
                if (i & (1 << bit)) != 0 {
                    this_address |= (1u64 << floating_bits[bit]) as usize;
                } else {
                    this_address &= !(1u64 << floating_bits[bit]) as usize;
                }
            }

            memory.write(this_address, value);
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
            for (pos, c) in mask.chars().rev().enumerate() {
                match c {
                    'X' => x_bits |= 1u64 << pos,
                    '1' => set_bits |= 1u64 << pos,
                    '0' => clear_bits &= !(1u64 << pos),
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
