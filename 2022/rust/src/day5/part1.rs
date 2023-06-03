use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, char, digit1, multispace1, newline, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, str) = alt((tag("   "), delimited(char('['), alpha1, char(']'))))(input)?;

    let result = match str {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

pub fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate)(input)
}

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}
fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            number,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crate_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    let mut crate_vertical: Vec<Vec<Option<&str>>> = vec![];
    for _ in 0..=crate_horizontal.iter().len() {
        crate_vertical.push(vec![]);
    }
    for vec in crate_horizontal.iter() {
        for (i, c) in vec.iter().enumerate() {
            crate_vertical[i].push(c.clone());
        }
    }
    let final_crates = crate_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();
    Ok((input, (final_crates, moves)))
}

pub fn without_drain(input: &str) -> String {
    let (_, (mut stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let items: Vec<_> = stacks[from.clone() as usize]
            .splice(0..(number.clone() as usize), Vec::new())
            .collect();
        for item in items.iter() {
            stacks[to.clone() as usize].insert(0, item);
        }
    }

    let result = stacks
        .iter()
        .filter_map(|stack| stack.first())
        .map(|item| *item)
        .collect::<Vec<&str>>()
        .join("");

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn it_work() {
        // Jetbrains's stupid auto remove trailing whitespace make me do this
        let test_input = fs::read_to_string("src/day5/test.txt").unwrap();
        assert_eq!(without_drain(test_input.as_str()), "CMZ");
    }
}
