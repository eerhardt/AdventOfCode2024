use std::{fs::File, io::{self, Read}};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut safe_count = 0;

    let mut lines = contents.lines();

    let mut done = false;
    while !done {
        match lines.next() {
            Some(line) => count_if_safe(&mut safe_count, line),
            None => done = true
        }
    }

    println!("safe count: {safe_count}");

    Ok(())
}

fn count_if_safe(safe_count: &mut i32, line: &str) {
    let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    let all_increasing_or_decreasing = are_all_increasing(&numbers) || are_all_decreasing(&numbers);
    let are_adjacent_gradual = are_adjacent_gradual(&numbers);

    if all_increasing_or_decreasing && are_adjacent_gradual {
        *safe_count += 1;
    }
    else {
        count_if_tolerable(safe_count, &numbers);
    }
}

fn are_all_increasing(numbers: &Vec<i32>) -> bool {
    for i in 1..numbers.len() {
        if numbers[i] <= numbers[i-1] {
            return false;
        }
    }

    return true;
}

fn are_all_decreasing(numbers: &Vec<i32>) -> bool {
    for i in 1..numbers.len() {
        if numbers[i] >= numbers[i-1] {
            return false;
        }
    }

    return true;
}

fn are_adjacent_gradual(numbers: &Vec<i32>) -> bool {
    for i in 1..numbers.len() {
        if (numbers[i] - numbers[i-1]).abs() > 3 {
            return false;
        }
    }

    return true;
}

fn count_if_tolerable(safe_count: &mut i32, numbers: &Vec<i32>) {
    for i in 0..numbers.len() {
        let numbers_without_i = copy_without_i(i, numbers);
        
        let all_increasing_or_decreasing = are_all_increasing(&numbers_without_i) || are_all_decreasing(&numbers_without_i);
        let are_adjacent_gradual = are_adjacent_gradual(&numbers_without_i);

        if all_increasing_or_decreasing && are_adjacent_gradual {
            *safe_count += 1;
            break;
        }
    }
}

fn copy_without_i(i: usize, numbers: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for j in 0..numbers.len() {
        if j != i {
            result.push(numbers[j]);
        }
    }
    return result;
}
