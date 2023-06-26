use std::fs;

mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    part1::run(input.as_str());
}
