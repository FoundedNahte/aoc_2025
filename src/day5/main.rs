use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

type EasyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn part1() -> EasyResult<()> {
    let file = File::open("src/day5/input.txt")?;
    let reader = BufReader::new(file);
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut consuming_ranges = true;
    let mut fresh = 0;
    'outer: for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            consuming_ranges = false;
            continue 'outer;
        }

        if consuming_ranges && line.contains("-") {
            let (valid_start, valid_end) = line
                .split("-")
                .collect::<Vec<&str>>()
                .chunks(2)
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect::<(String, String)>();
            ranges.push((valid_start.parse::<i64>()?, valid_end.parse::<i64>()?));
        } else {
            let num = line.parse::<i64>()?;
            for range in ranges.iter() {
                let (range_start, range_end) = *range;

                if num >= range_start && num <= range_end {
                    fresh += 1;
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", fresh);
    Ok(())
}

fn part2() -> EasyResult<()> {
    let file = File::open("src/day5/input.txt")?;
    let reader = BufReader::new(file);
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut fresh = 0;
    'outer: for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break 'outer;
        }

        if line.contains("-") {
            let (valid_start, valid_end) = line
                .split("-")
                .collect::<Vec<&str>>()
                .chunks(2)
                .map(|x| (x[0].to_string(), x[1].to_string()))
                .collect::<(String, String)>();
            ranges.push((valid_start.parse::<u64>()?, valid_end.parse::<u64>()?));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut i = 0;
    while i < ranges.len() {
        let mut j = i + 1;
        let mut new_range_end = ranges[i].1;
        while j < ranges.len() {
            if new_range_end < ranges[j].0 {
                break;
            }
            new_range_end = cmp::max(new_range_end, ranges[j].1);
            j += 1;
        }

        fresh += (new_range_end - ranges[i].0) + 1;
        i = j;
    }

    println!("{}", fresh);
    Ok(())
}

fn main() -> EasyResult<()> {
    part2()
}
