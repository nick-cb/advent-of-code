pub fn run(input: &str) -> String {
    let mut register_x: i32 = 1;
    let mut cycle = 0;
    let mut sprite_grid: Vec<Vec<bool>> = Vec::with_capacity(4);
    let mut sprite = Vec::with_capacity(40);
    sprite.fill_with(|| false);
    sprite_grid.fill_with(|| sprite.clone());
    sprite_grid[0][1] = true;

    for line in input.lines() {
        let result = line.split(" ").collect::<Vec<&str>>();
        let command = result.get(0).unwrap();
        match *command {
            "addx" => {
                let count = result.get(1).unwrap().parse::<i32>().unwrap();
                for _ in 0..2 {
                    cycle += 1;
                    // if cycle == 20 || cycle % 40 == 20 {
                    //     strengths.push(cycle * register_x);
                    // }
                }
                register_x += count;
            }
            "noop" => {
                cycle += 1;
                // if cycle == 20 || cycle % 40 == 20 {
                //     strengths.push(cycle * register_x);
                // }
            }
            _ => panic!("Invalid command"),
        };
    }

    todo!()
}

