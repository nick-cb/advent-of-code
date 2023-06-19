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
                .map(move |(j, current)| {
                    if i == 0 || i == rows.len() - 1 {
                        return 0;
                    }
                    if j == 0 || j == row.len() - 1 {
                        return 0;
                    }
                    let mut left = 0 as u32;
                    let mut right = 0 as u32;
                    let mut top = 0 as u32;
                    let mut bottom = 0 as u32;
                    for _j in (0..j.clone()).rev() {
                        if rows[i.clone()][_j] >= *current {
                            left += 1;
                            break;
                        }
                        left += 1;
                    }

                    for j_ in (j.clone() + 1)..rows[i].len() {
                        if rows[i.clone()][j_] >= *current {
                            right += 1;
                            break;
                        }
                        right += 1;
                    }
                    for _i in (0..i).rev() {
                        if rows[_i][j.clone()] >= *current {
                            top += 1;
                            break;
                        }
                        top += 1;
                    }
                    for i_ in (i + 1)..rows.len() {
                        if rows[i_][j.clone()] >= *current {
                            bottom += 1;
                            break;
                        }
                        bottom += 1;
                    }
                    // dbg!(format!("{}: {}, {}, {}, {}, {}", current, left, right, top, bottom, left* right* top* bottom));
                    left * right * top * bottom
                })
                .into_iter()
                .max()
                .unwrap()
        })
        .into_iter()
        .max()
        .unwrap();

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
        assert_eq!(run(INPUT), "8");
    }
}
