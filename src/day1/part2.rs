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
        assert_eq!(process_part2(input), "45000");
    }
}
