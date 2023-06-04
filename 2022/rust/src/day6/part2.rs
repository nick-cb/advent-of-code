use std::collections::BTreeSet;

pub fn run(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let (i, _) = chars
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();

    (i + 1 + 13).to_string()
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
    fn it_works() {
        assert_eq!(run(INPUT1), "19");
        assert_eq!(run(INPUT2), "23");
        assert_eq!(run(INPUT3), "23");
        assert_eq!(run(INPUT4), "29");
        assert_eq!(run(INPUT5), "26");
    }
}

