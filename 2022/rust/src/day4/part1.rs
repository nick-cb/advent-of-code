use std::ops::{Range, RangeInclusive};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use crate::day4::lib::section_assignment;

pub fn without_nom(input: &str) -> String {
    let result = input.lines().map(|line| {
        let sections = line.split(",").collect::<Vec<&str>>();
        let section_a = sections[0];
        let section_b = sections[1];

        let range_a = section_a.split("-").collect::<Vec<&str>>();
        let a_1 = range_a[0].parse::<u32>().unwrap();
        let a_2 = range_a[1].parse::<u32>().unwrap();

        let range_b = section_b.split("-").collect::<Vec<&str>>();
        let b_1 = range_b[0].parse::<u32>().unwrap();
        let b_2 = range_b[1].parse::<u32>().unwrap();

        return (a_1 >= b_1 && a_2 <= b_2) || (b_1 >= a_1 && b_2 <= a_2);
    }).filter(|p| *p == true).count();

    result.to_string()
}

pub fn with_nom(input: &str) -> String {
    let (_, assignments) = section_assignment(input).unwrap();
    let result = assignments.into_iter().filter(|(range_a, range_b)| {
        let a_contain_b = range_a.clone().into_iter().all(|num| range_b.contains(&num));
        let b_contain_a = range_b.clone().into_iter().all(|num| range_a.contains(&num));
        a_contain_b || b_contain_a
    }).count();

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
    fn without_nom_work() {
        assert_eq!(without_nom(INPUT), "2");
    }

    #[test]
    fn with_nom_work() {
        assert_eq!(with_nom(INPUT), "2");
    }
}