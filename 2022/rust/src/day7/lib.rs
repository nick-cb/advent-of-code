use nom::IResult;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::branch::alt;
use nom::sequence::separated_pair;

#[derive(Debug)]
pub struct File<'a> {
    pub size: u32,
    pub name: &'a str,
}

#[derive(Debug)]
pub enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
pub enum Files<'a> {
    File {size: u32, name: &'a str},
    Dir(&'a str)
}

#[derive(Debug)]
pub enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>)
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(nom::character::complete::u32,
                                               tag(" "),
                                               is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    Ok((input, Files::File {size, name}))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    Ok((input, op))
}

pub fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmds))
}
