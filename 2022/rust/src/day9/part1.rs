use std::collections::HashSet;
use crate::day9::lib;
use crate::day9::lib::Point;

pub fn run(input: &str) -> String {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited = HashSet::from([tail.clone()]);
    let (_, direction_vec) = lib::parse_line(input).unwrap();
    for Point { x, y } in direction_vec {
        let value = if x != 0 { x } else { y };
        for _ in 0..value.abs() {
            let prev = head.clone();
            if x != 0 {
                head.x += x.signum();
            } else {
                head.y += y.signum();
            }
            let distance = (((head.x - tail.x).pow(2) + (head.y - tail.y).pow(2)) as f32).sqrt();
            if distance >= 2.0 {
                tail.x = prev.x;
                tail.y = prev.y;
                visited.insert(prev);
            }
        }
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
