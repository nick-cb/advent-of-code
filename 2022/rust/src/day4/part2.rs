use crate::day4::lib::section_assignment;

pub fn run(input: &str) -> String {
    let (_, assignments) = section_assignment(input).unwrap();
    let result = assignments
        .into_iter()
        .filter(|(range_a, range_b)| {
            range_a
                .clone()
                .into_iter()
                .any(|num| range_b.contains(&num))
        })
        .count();

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_work() {
        assert_eq!(run(INPUT), "4");
    }
}
