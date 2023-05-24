use crate::day3::lib::get_letter_scores;

pub fn run(input: &str) -> String {
    let letter_scores = get_letter_scores();

    let result: usize = input.lines().map(|line| {
        let sack_len =  line.len() / 2;
        let compartment_a = &line[0..sack_len];
        let compartment_b = &line[sack_len..(sack_len * 2)];

        let common_char = compartment_a.chars().find(|c| compartment_b.contains(*c)).unwrap();

        letter_scores.get(&common_char).unwrap()
    }).sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn it_work() {
        let result = run(INPUT);
        assert_eq!(result, "157");
    }
}
