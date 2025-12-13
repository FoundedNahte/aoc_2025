use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type EasyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn part1() -> EasyResult<()> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let file = File::open("src/day4/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let row = line?;
        let row_vec = row.chars().map(|x| x == '@').collect::<Vec<bool>>();
        grid.push(row_vec);
    }

    let mut num_of_rolls = 0;
    let grid_size_y = grid.len();
    let grid_size_x = grid[0].len();
    for i in 0..grid_size_y {
        for j in 0..grid_size_x {
            let dirs: [i32; 3] = [-1, 0, 1];
            let mut adjacent_rolls = 0;
            if !grid[i][j] {
                continue;
            }
            for k in dirs {
                for m in dirs {
                    let new_i = i as i32 + k;
                    let new_j = j as i32 + m;
                    if (new_i < 0 || new_i >= grid_size_y as i32)
                        || (new_j < 0 || new_j >= grid_size_x as i32)
                    {
                        continue;
                    }

                    if let Some(row) = grid.get(new_i as usize)
                        && let Some(pos) = row.get(new_j as usize)
                        && *pos
                    {
                        adjacent_rolls += 1;
                    }
                }
            }

            if adjacent_rolls < 5 {
                num_of_rolls += 1;
            }
        }
    }
    println!("{}", num_of_rolls);
    Ok(())
}

fn part2() -> EasyResult<()> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let file = File::open("src/day4/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let row = line?;
        let row_vec = row.chars().map(|x| x == '@').collect::<Vec<bool>>();
        grid.push(row_vec);
    }

    let mut num_of_rolls = 0;
    let grid_size_y = grid.len();
    let grid_size_x = grid[0].len();
    let mut done = false;
    while !done {
        let mut count = 0;
        for i in 0..grid_size_y {
            for j in 0..grid_size_x {
                let dirs: [i32; 3] = [-1, 0, 1];
                let mut adjacent_rolls = 0;
                if !grid[i][j] {
                    continue;
                }
                for k in dirs {
                    for m in dirs {
                        let new_i = i as i32 + k;
                        let new_j = j as i32 + m;
                        if (new_i < 0 || new_i >= grid_size_y as i32)
                            || (new_j < 0 || new_j >= grid_size_x as i32)
                        {
                            continue;
                        }

                        if let Some(row) = grid.get(new_i as usize)
                            && let Some(pos) = row.get(new_j as usize)
                            && *pos
                        {
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 5 {
                    count += 1;
                    grid[i][j] = false;
                }
            }
        }
        if count == 0 {
            done = true;
        }

        num_of_rolls += count;
    }
    println!("{}", num_of_rolls);
    Ok(())
}

fn main() -> EasyResult<()> {
    part1()
}
