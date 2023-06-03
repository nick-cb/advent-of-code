pub fn run(input: &str ) -> String {
    let mut index: usize = 0;
    for (i, char) in input.chars().enumerate() {
        if i > input.len() - 4 || (i > &index + 3)  {
            break;
        }
        let sub = &input[i..=&i+3];
        for (j, sub_char) in sub.chars().skip(1).enumerate() {
            if char == sub_char {
                if i + j > &index + 3 {
                    break;
                }
                index = (&i + 1).clone();
                break;
            }
        }
    }
    (&index + 4).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn it_work() {
        assert_eq!(run(INPUT1), "7");
        assert_eq!(run(INPUT2), "5");
        assert_eq!(run(INPUT3), "6");
        assert_eq!(run(INPUT4), "10");
        assert_eq!(run(INPUT5), "11");
    }
}