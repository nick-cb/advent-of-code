use crate::day2::lib::Move;

pub fn run(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();

            moves[1] as u32
                + match (moves[0], moves[1]) {
                    (Move::Rock, Move::Rock) => 3,
                    (Move::Rock, Move::Paper) => 6,
                    (Move::Rock, Move::Scissors) => 0,
                    (Move::Paper, Move::Paper) => 3,
                    (Move::Paper, Move::Scissors) => 6,
                    (Move::Paper, Move::Rock) => 0,
                    (Move::Scissors, Move::Scissors) => 3,
                    (Move::Scissors, Move::Rock) => 6,
                    (Move::Scissors, Move::Paper) => 0,
                }
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_work() {
        let result = run(INPUT);
        assert_eq!(result, "15");
    }
}

