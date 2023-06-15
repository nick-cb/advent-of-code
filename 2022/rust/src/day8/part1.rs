pub fn run(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let rows = &lines.iter().map(|line| {
        line.chars().map(|c| c.parse::<u32>()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>();


    let total_visible: u32 = 0;
    for i in 0..rows.len() {
        if i == 0 || i == &rows.len() {
            total_visible += &rows[i].len();
            continue;
        }
        total_visible += 2;
        for j in 1..rows[i].len() - 1 {
            let current = rows[i][j];
            if j == 0 || j == rows[i].len() - 1 {
                continue;
            }
            if rows[i][j -1] < current || rows[i][j + 1] < current || rows[i -1][j] < current || row[i + 1][j] < current {
                total_visible += 1;
            }
        }
    }
    "nothing".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn it_works() {
        assert_eq!(run(INPUT), "1");
    }
}
