use std::fs;
mod lib;
mod part1;
mod part2;

pub fn run() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();
    println!("Part 1 - Without nom: {}", part1::without_nom(input.as_str()));
    println!("Part 1 - With nom: {}", part1::with_nom(input.as_str()));
    println!("Part 2 - With nom: {}", part2::run(input.as_str()));
}