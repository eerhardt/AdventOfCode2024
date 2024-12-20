use std::{fs::File, io::{self, Read}};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    
    let mut chars: Vec<Vec<char>> = Vec::new();

    let mut lines = contents.lines();
    let mut done = false;
    while !done {
        match lines.next() {
            Some(line) => chars.push(create_letter_line(line)),
            None => done = true
        }
    }
    
    let result = count_xmas(&chars);
    println!("result xmas: {result}");

    let result = count_x_mas(&chars);
    println!("result x-mas: {result}");

    Ok(())
}

fn create_letter_line(line: &str) -> Vec<char> {
    let mut result = Vec::new();
    for c in line.chars() {
        result.push(c);
    }
    
    return result;
}

fn count_xmas(chars: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            count += check_xmas(x, y, chars);
        }
    }

    return count;
}

fn check_xmas(x: usize, y: usize, chars: &Vec<Vec<char>>) -> i32 {
    if chars[y][x] != 'X' {
        // return early if this spot doesn't have an X
        return 0;
    }

    let mut result = 0;
    // check going upwards
    if y >= 3 {
        // diagonal up left
        if x >= 3 &&  chars[y - 3][x - 3] == 'S' && chars[y - 2][x - 2] == 'A' && chars[y - 1][x - 1] == 'M' {
            result += 1;
        }
        // up
        if chars[y - 3][x] == 'S' && chars[y - 2][x] == 'A' && chars[y - 1][x] == 'M' {
            result += 1;
        }
        // diagonal up right
        if x < chars[y].len() - 3 && chars[y - 3][x + 3] == 'S' && chars[y - 2][x + 2] == 'A' && chars[y - 1][x + 1] == 'M' {
            result += 1;
        }
    }

    // left
    if x >= 3 && chars[y][x - 3] == 'S' && chars[y][x - 2] == 'A' && chars[y][x - 1] == 'M' {
        result += 1;
    }

    // right
    if x < chars[y].len() - 3 && chars[y][x + 3] == 'S' && chars[y][x + 2] == 'A' && chars[y][x + 1] == 'M' {
        result += 1;
    }

    // check going downwards
    if y < chars.len() - 3 {
        // diagonal down left
        if x >= 3 &&  chars[y + 3][x - 3] == 'S' && chars[y + 2][x - 2] == 'A' && chars[y + 1][x - 1] == 'M' {
            result += 1;
        }
        // down
        if chars[y + 3][x] == 'S' && chars[y + 2][x] == 'A' && chars[y + 1][x] == 'M' {
            result += 1;
        }
        // diagonal down right
        if x < chars[y].len() - 3 && chars[y + 3][x + 3] == 'S' && chars[y + 2][x + 2] == 'A' && chars[y + 1][x + 1] == 'M' {
            result += 1;
        }
    }


    return result;
}

fn count_x_mas(chars: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            count += check_x_mas(x, y, chars);
        }
    }

    return count;
}

fn check_x_mas(x: usize, y: usize, chars: &Vec<Vec<char>>) -> i32 {
    if chars[y][x] != 'A' {
        // return early if this spot doesn't have an A
        return 0;
    }

    let mut result = 0;
    if y >= 1 && y < chars.len() - 1 && x >= 1 && x < chars[y].len() - 1 {
        if chars[y - 1][x - 1] == 'S' && chars[y + 1][x + 1] == 'M' ||
           chars[y - 1][x - 1] == 'M' && chars[y + 1][x + 1] == 'S'  {
            result += 1;
        }

        if chars[y - 1][x + 1] == 'S' && chars[y + 1][x - 1] == 'M' ||
           chars[y - 1][x + 1] == 'M' && chars[y + 1][x - 1] == 'S'  {
            result += 1;
        }
    }

    if result > 1 {
        return 1;
    }
    else {
        return 0;
    }
}