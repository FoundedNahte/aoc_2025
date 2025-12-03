use std::{cmp, fs::File, i64::MIN, io::Read, slice::SplitN};

type EasyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn split_s(s: String, parts: usize) -> Vec<String> {
    let mut splits = Vec::new();
    let size = s.len() / parts;
    let mut curr = 0;
    for _ in 0..parts {
        splits.push(s[curr..cmp::min(s.len(), curr + size)].to_string());
        curr += size;
    }

    splits
}

fn check_num_part2(num: &i64) -> EasyResult<bool> {
    let ori_string = num.to_string();
    for i in 2..=ori_string.len() {
        if ori_string.len() % i != 0 {
            continue;
        }
        let splits = split_s(ori_string.clone(), i);
        let mut internal = true;
        for k in 1..splits.len() {
            if splits[0] != splits[k] {
                internal = false;
                break;
            }
        }

        if internal {
            return Ok(true);
        }
    }

    Ok(false)
}

fn check_num_part1(num: &i64) -> EasyResult<bool> {
    let mut left_string= num.to_string();
    let right_string = left_string.split_off(left_string.len() / 2);
    Ok(left_string == right_string)
}

fn main() -> EasyResult<()> {
    let mut file = File::open("src/day2/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents = contents.split(",").map(|x| String::from(x)).collect::<Vec<String>>();
    let mut total: i64 = 0;
    for pair in contents {
        let ids = pair.split('-').map(|x| String::from(x)).collect::<Vec<String>>(); 
        let (id1, id2) = (ids[0].parse::<i64>()?, ids[1].parse::<i64>()?);

        for i in id1..=id2 {
            if check_num_part1(&i)? {
                total += i;
            } 
        }
    }
    println!("{total}");
    Ok(())
}