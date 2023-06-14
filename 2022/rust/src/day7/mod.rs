use std::fs;

mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();
    println!("Part1: {}", part1::run(input.as_str()));
}