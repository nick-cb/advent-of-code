use std::fs;

pub fn process_part1(input: &str) -> String {
    let result = input.split("\n\n").map(|elf_load| {
        elf_load.lines().map(|item| {
            item.parse::<u32>().unwrap()
        }).sum::<u32>()
    }).max().unwrap();

    return result.to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut result = input.split("\n\n").map(|elf_load| {
        elf_load.lines().map(|item| {
            item.parse::<u32>().unwrap()
        }).sum::<u32>()
    }).collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();

    sum.to_string()
}

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());

    let contents = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part1(contents.as_str()));
    println!("{}", process_part2(contents.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(process_part1(input), "24000");
        assert_eq!(process_part2(input), "45000");
    }
}