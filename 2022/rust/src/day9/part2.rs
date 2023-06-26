use crate::day9::lib;
use crate::day9::lib::Point;
use std::collections::HashSet;

pub fn run(input: &str) -> String {
    // let (_, direction_vec) = parse_line_with_direction(input).unwrap();
    let mut knot_vec = vec![
        Point { x: 0, y: 0 }, // H
        Point { x: 0, y: 0 }, // 8
        Point { x: 0, y: 0 }, // 7
        Point { x: 0, y: 0 }, // 6
        Point { x: 0, y: 0 }, // 5
        Point { x: 0, y: 0 }, // 4
        Point { x: 0, y: 0 }, // 3
        Point { x: 0, y: 0 }, // 2
        Point { x: 0, y: 0 }, // 1
        Point { x: 0, y: 0 }, // T
    ];
    let (_, direction_vec) = lib::parse_line(input).unwrap();
    let mut visited = HashSet::from([Point { x: 0, y: 0 }]);
    for Point { x, y } in direction_vec {
        let value = if x != 0 { x } else { y };
        for _ in 0..value.abs() {
            for index in 0..=9 {
                if index == 0 {
                    let knot: &mut Point = knot_vec.get_mut(0).unwrap();
                    if x != 0 {
                        knot.x += x.signum();
                    } else {
                        knot.y += y.signum();
                    }
                    continue;
                }
                let [head, knot] = knot_vec.get_many_mut([index - 1, index]).unwrap();
                let distance =
                    (((head.x - knot.x).pow(2) + (head.y - knot.y).pow(2)) as f32).sqrt();
                if distance >= 2.0 {
                    if head.x == knot.x {
                        knot.y += (head.y - knot.y).signum();
                    }
                    if head.y == knot.y {
                        knot.x += (head.x - knot.x).signum();
                    }
                    if head.x != knot.x && head.y != knot.y {
                        knot.x += (head.x - knot.x).signum();
                        knot.y += (head.y - knot.y).signum();
                    }
                    if index == 9 {
                        visited.insert(knot.clone());
                    }
                }
            }
        }
    }
    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn it_works() {
        assert_eq!(run(INPUT_1), "1");
        assert_eq!(run(INPUT_2), "36");
    }
}

                                                                                                                    