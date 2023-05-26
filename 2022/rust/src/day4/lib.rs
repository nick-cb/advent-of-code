use std::ops::RangeInclusive;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

pub fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = nom::character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;

    Ok((input, start..=end))
}
pub fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    /*    let (input, start) = sections(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, end) = sections(input)?;*/
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, (start, end)))
}
pub fn section_assignment(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;
    Ok((input, ranges))
}
