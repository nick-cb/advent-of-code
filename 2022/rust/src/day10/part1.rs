pub fn run(input: &str) -> String {
    let mut register_x: i32 = 1;
    let mut strengths: Vec<i32> = vec![];
    let mut cycle = 0;
    for line in input.lines() {
        let result = line.split(" ").collect::<Vec<&str>>();
        let command = result.get(0).unwrap();
        match *command {
            "addx" => {
                let count = result.get(1).unwrap().parse::<i32>().unwrap();
                for _ in 0..2 {
                    cycle += 1;
                    if cycle == 20 || cycle % 40 == 20 {
                        strengths.push(cycle * register_x);
                    }
                }
                register_x += count;
            },
            "noop" => {
                cycle += 1;
                if cycle == 20 || cycle % 40 == 20 {
                    strengths.push(cycle * register_x);
                }
            },
            _ => panic!("Invalid command"),
        };

    };

    let result = strengths.into_iter().sum::<i32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn it_works() {
        assert_eq!(run(INPUT), "13140");
    }
}


/*
noop
addx 3
addx -5

noop => =1=
addx = =2=..=3=
*/