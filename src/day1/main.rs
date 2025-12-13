use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type EasyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn part1() -> EasyResult<i32> {
    let file = File::open("src/day1/input.txt")?;
    let reader = BufReader::new(file);
    let mut starting_num = 50;
    let mut count = 0;

    for line in reader.lines() {
        let combo_string = line?;
        let (m, num) = combo_string.split_at(1);
        let num = num.parse::<i32>()?;
        let mut normalized_turn = num;
        if m == "L" {
            normalized_turn = 100 - num;
        }
        starting_num = (starting_num + normalized_turn) % 100;

        if starting_num == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn part2() -> EasyResult<i32> {
    let file = File::open("src/day1/input.txt")?;
    let reader = BufReader::new(file);

    let mut starting_num = 50;
    let mut count = 0;
    for line in reader.lines() {
        let combo_string = line?;
        let (m, num) = combo_string.split_at(1);
        let num = num.parse::<i32>()?;

        for _ in 0..num {
            if m == "L" {
                starting_num -= 1;
            } else {
                starting_num += 1;
            }

            if starting_num < 0 {
                starting_num += 100;
            } else {
                starting_num %= 100;
            }

            if starting_num == 0 {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn main() -> EasyResult<()> {
    println!("{}", part2()?);
    Ok(())
}
