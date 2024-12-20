use std::{fs::File, io::{self, Read}};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;
    for (_, [digit1, digit2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        result += digit1.parse::<i32>().unwrap() * digit2.parse::<i32>().unwrap();
    }

    println!("result 1: {result}");

    let result2 = get_result_with_do_dont(&contents);

    println!("result 2: {result2}");

    Ok(())
}

fn get_result_with_do_dont(mut contents: &str) -> i32 {
    
    let mut result = 0;

    let mut in_do = true;
    while contents.len() > 0 {

        if contents.starts_with("don't()") {
            in_do = false;
            contents = &contents[7..];
            continue;
        }
        if contents.starts_with("do()") {
            in_do = true;
            contents = &contents[4..];
            continue;
        }
        if in_do && contents.starts_with("mul(") {
            contents = &contents[4..];
            
            match match_mul_args(&contents) {
                Some((left, right, step)) => {
                    result += left * right; 
                    contents = &contents[step..];
                    continue;
                }
                None => continue
            }
        }

        contents = &contents[1..];
    }

    return result;
}

fn match_mul_args(contents: &str) -> Option<(i32, i32, usize)> {
    let mut first_index = 0;

    while contents.len() > first_index && contents.chars().nth(first_index).unwrap().is_numeric() {
        first_index += 1;
    }

    let first = contents[0..first_index].parse::<i32>().unwrap();

    if contents.len() > first_index && contents[first_index..].starts_with(",") {

        let mut second_index = first_index + 1;
        while contents.len() > second_index && contents.chars().nth(second_index).unwrap().is_numeric() {
            second_index += 1;
        }
        let second = contents[first_index+1..second_index].parse::<i32>().unwrap();

        if contents.len() > second_index && contents[second_index..].starts_with(")") {
            return Some((first, second, second_index + 1));
        }
    }

    return None;
}