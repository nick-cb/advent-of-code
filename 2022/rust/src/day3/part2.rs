use crate::day3::lib::get_letter_scores;

pub fn without_array_chunk(input: &str) -> String {
    let letter_scores = get_letter_scores();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total_scores = 0;
    let mut i = 0;
    while i < lines.len() {
        let group_1 = lines[i];
        let group_2 = lines[&i + 1];
        let group_3 = lines[&i + 2];

        let common_char = group_1.chars().find(|c|
            group_2.contains(c.clone()) && group_3.contains(c.clone())
        ).unwrap();

        total_scores = total_scores + letter_scores.get(&common_char).unwrap();
        i = &i + 3;
    }

    total_scores.to_string()
}

pub fn with_array_chunk(input: &str) -> String {
    let letter_scores = get_letter_scores();
    let result = input.lines().array_chunks::<3>().map(|[a, b, c]| {
        let common_char = a.chars().find(|a_char| b.contains(a_char.clone()) && c.contains(a_char.clone())).unwrap();

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
    fn without_array_chunk_work() {
        assert_eq!(without_array_chunk(INPUT), "70");
    }

    #[test]
    fn with_array_chunk_work() {
        assert_eq!(with_array_chunk(INPUT), "70");
    }
}
