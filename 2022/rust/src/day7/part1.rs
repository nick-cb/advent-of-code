use std::collections::BTreeMap;
use crate::day7::lib;
use crate::day7::lib::{Cd, File, Files, Operation};

pub fn run(input: &str) -> String {
    let cmds = lib::commands(input).unwrap().1;
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
dbg!(&directories);
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