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

struct Point {
    x: i32,
    y: i32,
}

fn direction_parser(input: &str) -> IResult<&str, Point> {
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

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points) = separated_list1(newline, direction_parser)(input)?;

    Ok((input, points))
}

pub fn run(input: &str) -> String {
    let mut visited = 1;
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let (_, direction_vec) = parse_line(input).unwrap();

    for Point { x, y } in direction_vec {
        let value = if x != 0 { x } else { y };
        for _ in 0..value {
            let prev = Point {
                x: head.x.clone(),
                y: head.y.clone(),
            };
            if x != 0 {
                head.x += x.signum();
            } else {
                head.y += y.signum();
            }
            let distance = (((head.x - tail.x).pow(2) + (head.y - tail.y).pow(2)) as f32).sqrt();
            if distance > 1 as f32 {
                tail.x = prev.x;
                tail.y = prev.y;
                visited += 1;
            }
        }
    }
    visited.to_string()
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
