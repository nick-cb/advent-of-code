use std::collections::BTreeSet;

pub fn brute_force(input: &str) -> String {
    let mut index: usize = 0;
    for (i, char) in input.chars().enumerate() {
        if i > input.len() - 4 || (i > &index + 3) {
            break;
        }
        let sub = &input[i..=&i + 3];
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

pub fn using_windows(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    (sequence.0 + 1 + 3).to_string()
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
    fn brute_force_works() {
        assert_eq!(brute_force(INPUT1), "7");
        assert_eq!(brute_force(INPUT2), "5");
        assert_eq!(brute_force(INPUT3), "6");
        assert_eq!(brute_force(INPUT4), "10");
        assert_eq!(brute_force(INPUT5), "11");
    }

    #[test]
    fn window_works() {
        assert_eq!(using_windows(INPUT1), "7");
        assert_eq!(using_windows(INPUT2), "5");
        assert_eq!(using_windows(INPUT3), "6");
        assert_eq!(using_windows(INPUT4), "10");
        assert_eq!(using_windows(INPUT5), "11");
    }
}

