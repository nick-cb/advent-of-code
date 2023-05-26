use std::fs;
mod lib;
mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    println!("Part 1: {}", part1::run(input.as_str()));
    println!("Part 2: {}", part2::with_array_chunk(input.as_str()));
}

