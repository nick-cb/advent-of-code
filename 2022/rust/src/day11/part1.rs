use nom::{
    bytes::complete::tag,
    character::complete::{digit0, newline},
    multi::{many0, separated_list1},
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    multiplier: i32,
    divisible: i32,
    monkey1: usize,
    monkey2: usize,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = preceded(tag("Monkey"), digit0)(input)?;
    let mut monkey = Monkey {
        items: Vec::new(),
        multiplier: 0,
        divisible: 0,
        monkey1: 0,
        monkey2: 0,
    };

    let (input, _) = newline(input)?;
    let (input, item_list) = preceded(tag("Starting items:"), many0(digit0))(input)?;
    for item in item_list {
        monkey.items.push(item.parse::<i32>().unwrap());
    }
    let (input, _) = newline(input)?;
    let (input, multiplier) = preceded(tag("Operation: new = old * "), digit0)(input)?;
    monkey.multiplier = multiplier.parse::<i32>().unwrap();
    let (input, _) = newline(input)?;
    let (input, divisible) = preceded(tag("Test: divisible by "), digit0)(input)?;
    monkey.divisible = divisible.parse::<i32>().unwrap();
    let (input, _) = newline(input)?;
    let (input, monkey1) = preceded(tag("  If true: throw to monkey "), digit0)(input)?;
    monkey.monkey1 = monkey1.parse::<usize>().unwrap();
    let (input, _) = newline(input)?;
    let (input, monkey2) = preceded(tag("  If false: throw to monkey "), digit0)(input)?;
    monkey.monkey2 = monkey2.parse::<usize>().unwrap();

    Ok((input, monkey))
}

fn parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, list) = separated_list1(newline, parse_monkey)(input)?;

    Ok((input, list))
}

pub fn run(input: &str) -> String {
    let (input, monkeys) = parser(input).unwrap();
    dbg!(&monkeys);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn it_works() {
        assert_eq!(run(INPUT), "10605");
    }
}
