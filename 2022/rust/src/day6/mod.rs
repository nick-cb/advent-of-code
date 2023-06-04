use std::fs;

mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    println!("Part 1 - Brute force: {}", part1::brute_force(input.as_str()));
    println!("Part 1 - Using windows: {}", part1::using_windows(input.as_str()));
}