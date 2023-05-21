mod day1;
use std::fs;
use crate::day1::part1::process_part1;
use crate::day1::part2::process_part2;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());

    let contents = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part1(contents.as_str()));
    println!("{}", process_part2(contents.as_str()));
}