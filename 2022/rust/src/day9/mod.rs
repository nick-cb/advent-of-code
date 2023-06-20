use std::fs;

mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day9/input.txt").unwrap();
    println!("Part 1: {}", part1::run(&input));
}
