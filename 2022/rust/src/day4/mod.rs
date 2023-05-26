use std::fs;
mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    println!("Part 1: {}", part1::without_nom(input.as_str()));
    println!("Part 1: {}", part1::with_nom(input.as_str()));
}