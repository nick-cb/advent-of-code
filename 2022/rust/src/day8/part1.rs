pub fn run(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let rows = &lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result = rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, current)| {
                    if i == 0 || i.clone() == rows.len() - 1 {
                        return true;
                    }
                    if *j == 0 || *j == row.len() - 1 {
                        return true;
                    }
                    let mut left = true;
                    let mut top = true;
                    let mut bottom = true;
                    let mut right = true;
                    for _j in 0..j.clone() {
                        if rows[i.clone()][_j] >= **current {
                            left = false;
                        }
                    }

                    for j_ in (j.clone() + 1)..rows[i].len() {
                        if rows[i.clone()][j_] >= **current {
                            right = false;
                        }
                    }
                    for _i in 0..i {
                        if rows[_i][j.clone()] >= **current {
                            top = false;
                        }
                    }
                    for i_ in (i + 1)..rows.len() {
                        if rows[i_][j.clone()] >= **current {
                            bottom = false;
                        }
                    }
                    left || right || top || bottom
                })
                .map(|(_, val)| val)
                .collect::<Vec<&u32>>()
                .len()
        })
        .sum::<usize>();

    result.to_string()
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
        assert_eq!(run(INPUT), "21");
    }
}
