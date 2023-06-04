use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::not;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;

struct Dir {
    name: String,
    size: u32,
    dir_list: Vec<Dir>,
}

fn dir(input: &str) -> IResult<&str, &str> {
    let (input, _) = delimited(tag(" "), tag("cd"), tag(" "))(input)?;

    Ok((input, input))
}

fn file(input: &str) -> IResult<&str, &str> {
    let (input, _) = not(tag("dir"));
}

fn command(input: &str) -> IResult<&str, Dir> {
    let (input, _) = tag("$")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, command) = alt((tag("cd"),tag("ls")))(input)?;
    let mut dir = Dir {
        name: "unknown".to_string(),
        size: 0,
        dir_list: vec![]
    };
    if command == "cd" {
        let (dirname, _) = tag(" ")(input)?;
        dir.name = dirname.to_string();
    }

    if command == "ls" {
        separated_list1(newline, )
    }

    Ok(( input,  dir))
}

fn line(input: &str) -> IResult<&str, ()> {
    let (input, commands) = separated_list1(newline, command)(input)?;

    Ok((input, ()))
}

pub fn run(input: &str) -> String {
    "nothing".to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn it_works() {
        assert_eq!(run(INPUT), "95437");
    }
}