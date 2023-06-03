use std::fs;
mod part1;

pub fn run() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();
/*    println!(
        "Part 1 - Without drained: {}",
        part1::without_drain(input.as_str())
    );*/
    println!(
        "Part 1 - With drained: {}",
        part1::with_drained(input.as_str())
    );
}
