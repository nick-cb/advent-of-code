use nom::{character, IResult};
use nom::sequence::separated_pair;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;

pub enum Direction {
    R(Point),
    L(Point),
    U(Point),
    D(Point),
}

#[derive(Debug, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn direction_parser(input: &str) -> IResult<&str, Point> {
    let (input, (direction, value)) =
        separated_pair(character::complete::alpha0, tag(" "), digit1)(input)?;
    let point = match direction {
        "R" => Point {
            x: value.parse::<i32>().unwrap(),
            y: 0,
        },
        "L" => Point {
            x: -value.parse::<i32>().unwrap(),
            y: 0,
        },
        "U" => Point {
            x: 0,
            y: value.parse::<i32>().unwrap(),
        },
        "D" => Point {
            x: 0,
            y: -value.parse::<i32>().unwrap(),
        },
        _ => panic!("Invalid direction: {}", direction),
    };
    Ok((input, point))
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points) = separated_list1(newline, direction_parser)(input)?;

    Ok((input, points))
}

pub fn direction_parser_2(input: &str) -> IResult<&str, Direction> {
    let (input, (direction, value)) =
        separated_pair(character::complete::alpha0, tag(" "), digit1)(input)?;
    let point = match direction {
        "R" => Direction::R(Point {
            x: value.parse::<i32>().unwrap(),
            y: 0,
        }),
        "L" => Direction::L( Point {
            x: -value.parse::<i32>().unwrap(),
            y: 0,
        }),
        "U" => Direction::U(Point {
            x: 0,
            y: value.parse::<i32>().unwrap(),
        }),
        "D" => Direction::D( Point {
            x: 0,
            y: -value.parse::<i32>().unwrap(),
        }),
        _ => panic!("Invalid direction: {}", direction),
    };
    Ok((input, point))
}

pub fn parse_line_with_direction(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, points) = separated_list1(newline, direction_parser_2)(input)?;

    Ok((input, points))
}
