use std::fs;

mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day8/input.txt").unwrap();
    println!("Part1 - Brute force: {}", part1::run(&input));
}
