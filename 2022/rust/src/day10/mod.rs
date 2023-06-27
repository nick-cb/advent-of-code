use std::fs;

mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day10/input.txt").unwrap();
    println!("Part1: {}", part1::run(input.as_str()));
    part2::run(input.as_str());
}
