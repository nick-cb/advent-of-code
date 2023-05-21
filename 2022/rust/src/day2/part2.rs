use crate::day2::lib::Move;

pub fn run(input: &str) -> String {
    let result: u32 = input.lines().map(|line| {
        let moves: Vec<&str> = line.split(" ").collect();
        let opponent_move = moves[0].parse::<Move>().unwrap();

        match moves[1] {
            "X" => {
                let our_move = match opponent_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                };

                our_move as u32
            },
            "Y" => opponent_move as u32 + 3,
            "Z" => {
                6 + match opponent_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                } as u32
            },
                _ => panic!("Not a known move"),
        }
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part2_works() {
        let result = run(INPUT);
        assert_eq!(result, "12");
    }
}