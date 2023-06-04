use std::fs;

mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day6/input.txt").unwrap();
    println!("Part 1 - Brute force: {}", part1::brute_force(input.as_str()));
    println!("Part 1 - Using windows: {}", part1::using_windows(input.as_str()));
    println!("Part 2 - Using windows: {}", part2::run(input.as_str()));
}