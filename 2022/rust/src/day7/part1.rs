use std::collections::BTreeMap;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File {size: u32, name: &'a str},
    Dir(&'a str)
}

#[derive(Debug)]
enum Operation<'a> {
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

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmds))
}

pub fn run(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
            }
            Operation::Ls(files) => {
                directories.entry(context.iter()
                    .cloned()
                    .intersperse("/")
                    .collect::<String>()
                ).or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File {size, name} => {
                            directories.entry(context.iter().cloned().intersperse("/").collect::<String>()).and_modify(|vec| {
                                vec.push(File {
                                    size: size.clone(),
                                    name,
                                })
                            });
                        }
                        Files::Dir(__) => ()
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files.iter().map(|File {size, ..}| size).sum::<u32>();
        for i in 0..dirs.len() {
            sizes.entry((&dirs[0..=i]).iter()
                .cloned()
                .intersperse("/")
                .collect::<String>()
            ).and_modify(|v| *v += size.clone()).or_insert(size.clone());
        }
    }

    sizes.iter().filter(|(_, &size) | size < 100000).map(|(_, size) | size).sum::<u32>().to_string()
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