use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

type EasyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn part1() -> EasyResult<()> {
    let file = File::open("src/day3/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_joltage = 0;

    for line in reader.lines() {
        let line = line?;
        let digits = line.chars().collect::<Vec<char>>();

        let mut highest = 0;
        for left_idx in 0..line.len() {
            for right_idx in (left_idx + 1..line.len()).rev() {
                let mut num = String::new();
                num.push(digits[left_idx]);
                num.push(digits[right_idx]);

                let num = num.parse::<i32>()?;
                highest = cmp::max(highest, num);
            }
        }

        total_joltage += highest;
    }
    println!("Total Joltage: {}", total_joltage);
    Ok(())
}

fn part2() -> EasyResult<()> {
    let file = File::open("src/day3/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_joltage: u64 = 0;

    for line in reader.lines() {
        let line = line?;
        let digits = line
            .chars()
            .map(|x| x.to_digit(10).expect("Failed to convert"))
            .collect::<Vec<u32>>();

        let mut trackers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        //let mut trackers = [0, 1, 2, 3, 4, 5];
        let mut curr_tracker = 0;
        while curr_tracker < 12 {
            let mut index = 0;
            if curr_tracker > 0 {
                index = trackers[curr_tracker - 1] + 1;
            }

            // Backtracking
            if index >= digits.len() {
                let mut temp_tracker = curr_tracker - 1;
                // Find tracker behind that has room
                while temp_tracker > 0 && trackers[temp_tracker] == trackers[temp_tracker - 1] + 1 {
                    temp_tracker -= 1;
                }

                // Once we find tracker, find new max and work again
                let end = trackers[temp_tracker];
                let mut start = 0;
                if temp_tracker != 0 {
                    start = trackers[temp_tracker - 1] + 1;
                }

                let mut backtracked_index = 0;
                let mut highest = 0;
                for i in start..end {
                    if digits[i] > highest {
                        highest = digits[i];
                        backtracked_index = i;
                    }
                }
                trackers[temp_tracker] = backtracked_index;
                curr_tracker = temp_tracker + 1;
                continue;
            }

            let mut highest_index = 0;
            let mut highest = 0;
            for i in cmp::max(index, trackers[curr_tracker])..digits.len() {
                if digits[i] > highest {
                    highest = digits[i];
                    highest_index = i;
                }
            }

            trackers[curr_tracker] = highest_index;
            curr_tracker += 1;
        }

        let mut joltage = String::new();
        for tracker in trackers {
            joltage.push(char::from_digit(digits[tracker], 10).unwrap())
        }
        total_joltage += joltage.parse::<u64>()?;
    }

    println!("{}", total_joltage);
    Ok(())
}

fn main() -> EasyResult<()> {
    part2()
}
