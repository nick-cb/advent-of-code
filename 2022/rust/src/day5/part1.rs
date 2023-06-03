use crate::day5::lib::{crates, Move};

pub fn without_drain(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let stack = &mut stacks[from.clone() as usize];
        let items = stack
            .split_off(stack.len() - number.clone() as usize);

        for item in items.iter().rev() {
            stacks[to.clone() as usize].push(item)
        }
    }

    let result = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .map(|item| *item)
        .collect::<Vec<&str>>()
        .join("");

    result.to_string()
}

pub fn with_drained(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let len = stacks[from.clone() as usize].len();
        let drained = stacks[from.clone() as usize]
            .drain((len - number.clone() as usize)..)
            .rev()
            .collect::<Vec<&str>>();
        // dbg!(&drained);
        for c in drained.iter() {
            stacks[to.clone() as usize].push(c);
        }
    }

    let result: String = stacks.iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => ""
        }).collect();

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn without_drain_work() {
        // Jetbrains's stupid auto remove trailing whitespace make me do this
        let test_input = fs::read_to_string("src/day5/test.txt").unwrap();
        assert_eq!(without_drain(test_input.as_str()), "CMZ");
    }

    #[test]
    fn with_drained_work() {
        let test_input = fs::read_to_string("src/day5/test.txt").unwrap();
        assert_eq!(with_drained(test_input.as_str()), "CMZ");
    }
}
