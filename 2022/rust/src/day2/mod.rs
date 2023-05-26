use std::fs;
pub mod lib;
mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    println!("Part 1: {}", part1::run(input.as_str()));
    println!("Part 2: {}", part2::run(input.as_str()));
}

