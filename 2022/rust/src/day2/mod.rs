use std::fs;
pub mod lib;
mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    println!("Part1: {}", part1::run(input.as_str()));
}