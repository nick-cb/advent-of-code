use std::hash::{Hasher};
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{digit1, newline},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    R(i32),
    L(i32),
    U(i32),
    D(i32),
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    let (input, (direction, value)) =
        separated_pair(character::complete::alpha0, tag(" "), digit1)(input)?;
    let point = match direction {
        "R" => Direction::R(value.parse::<i32>().unwrap()),
        "L" => Direction::L(value.parse::<i32>().unwrap()),
        "U" => Direction::U(value.parse::<i32>().unwrap()),
        "D" => Direction::D(value.parse::<i32>().unwrap()),
        _ => panic!("Invalid direction: {}", direction),
    };
    Ok((input, point))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = separated_list1(newline, direction_parser)(input)?;

    Ok((input, directions))
}

pub fn run(input: &str) -> String {
    let mut visited: Vec<Point> = vec![];
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    visited.push(Point {
        x: 0,
        y: 0
    });
    let (_, direction_vec) = parse_line(input).unwrap();
    for direction in direction_vec {
        match direction {
            Direction::R(count) => {
                for _ in 0..count {
                    head.x += 1;
                    if head.x - tail.x < 2 {
                        continue;
                    }
                    if tail.x != head.x {
                        tail.y = head.y;
                    }
                    if tail.x == head.x - 1 {
                        break;
                    }
                    tail.x += 1;
                    let mut exist = false;
                    for Point {x, y} in &mut visited {
                        if x == &tail.x && y == &tail.y {
                            exist = true;
                            break;
                        }
                    }
                    if !exist {
                        visited.push(Point {x: tail.x.clone(), y: tail.y.clone()});
                    }
                }
/*                head.x += count;
                if tail.x != head.x {
                    tail.y = head.y;
                }
                let next_point = head.x - 1;
                visited += next_point - tail.x;
               tail.x = next_point; */
            },
            Direction::L(count) => {
                for _ in 0..count {
                    head.x -= 1;
                    if tail.x - head.x < 2 {
                        continue;
                    }
                    if tail.x != head.x {
                        tail.y = head.y;
                    }
                    if tail.x == head.x + 1 {
                        break;
                    }
                    tail.x -= 1;
                    let mut exist = false;
                    for Point {x, y} in &mut visited {
                        if x == &tail.x && y == &tail.y {
                            exist = true;
                            break;
                        }
                    }
                    if !exist {
                        visited.push(Point {x: tail.x.clone(), y: tail.y.clone()});
                    }
                }
            }
            Direction::U(count) => {
                for _ in 0..count {
                    head.y += 1;
                    if head.y - tail.y < 2 {
                        continue;
                    }
                    if tail.y != head.y {
                        tail.x = head.x;
                    }
                    if tail.y == head.y - 1 {
                        break;
                    }
                    tail.y += 1;
                    let mut exist = false;
                    for Point {x, y} in &mut visited {
                        if x == &tail.x && y == &tail.y {
                            exist = true;
                            break;
                        }
                    }
                    if !exist {
                        visited.push(Point {x: tail.x.clone(), y: tail.y.clone()});
                    }
                }
            }
            Direction::D(count) => {
                for _ in 0..count {
                    head.y -= 1;
                    if tail.y - head.y < 2 {
                        continue;
                    }
                    if tail.y != head.y {
                        tail.x = head.x;
                    }
                    tail.y -= 1;
                    let mut exist = false;
                    for Point {x, y} in &mut visited {
                        if x == &tail.x && y == &tail.y {
                            exist = true;
                            break;
                        }
                    }
                    if !exist {
                        visited.push(Point {x: tail.x.clone(), y: tail.y.clone()});
                    }
                }
            }
        };
    }
    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_run() {
        assert_eq!(run(INPUT), "13");
    }
}
