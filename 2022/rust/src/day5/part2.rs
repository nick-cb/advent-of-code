use crate::day5::lib::{crates, Move};

pub fn with_drained(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let len = stacks[from.clone() as usize].len();
        let items = &stacks[from.clone() as usize].drain((len - number.clone() as usize)..)
            .collect::<Vec<&str>>();
        stacks[to.clone() as usize].extend(items);
    }
    let result: String = stacks.iter().map(|stack| match stack.last() {
        Some(value) => value,
        None => ""
    }).collect();

    result.to_string()
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn with_drained_work() {
        let test_input = fs::read_to_string("src/day5/test.txt").unwrap();
        assert_eq!(with_drained(test_input.as_str()), "MCD");
    }
}