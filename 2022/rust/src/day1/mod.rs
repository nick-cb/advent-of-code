use std::fs;
use crate::day1::part1::process_part1;
use crate::day1::part2::process_part2;

pub mod part1;
pub mod part2;

pub fn run() {
    println!("{}", std::env::current_dir().unwrap().display());

    let contents = fs::read_to_string("../input.txt").unwrap();
    println!("{}", process_part1(contents.as_str()));
    println!("{}", process_part2(contents.as_str()));
}