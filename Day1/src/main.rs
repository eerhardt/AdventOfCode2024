use std::{collections::HashMap, fs::File, io::{self, Read}, vec};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    //println!("File contents: {}", contents);

    // parse the contents of the file, the input is 2 integers separated by 3 spaces
    let numbers: Vec<i32> = contents.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    // foreach number in numbers
    // for number in numbers {
    //     println!("number: {}", number);
    // }   
    
    // create an array
    let mut number1 = vec![0; numbers.len() / 2];
    let mut number2 = vec![0; number1.len()];

    let mut index = 0;
    let mut even = true;
    for number in numbers {
        if even {
            number1[index] = number;
        } else {
            number2[index] = number;
            index += 1;
        }

        even = !even;
    }

    // for number in number1 {
    //     println!("number1: {}", number);
    // }  

    // for number in number2 {
    //     println!("number2: {}", number);
    // }  

    number1.sort();
    number2.sort();

    let mut distance = vec![0; number1.len()];
    for i in 0..number1.len() {
        distance[i] = (number1[i] - number2[i]).abs();
    }

    let mut result1 = 0;
    for i in 0..distance.len() {
        result1 += distance[i];
    }

    println!("result1: {}", result1);

    // -----------------------
    // 2nd problem - similarity
    // -----------------------

    let mut number2_counts = HashMap::new();

    let mut i = 0;
    while i < number2.len() {
        let cur = number2[i];

        let mut current_count = 1;
        while i < number2.len() - 1 && cur == number2[i+1] {
            current_count += 1;
            i += 1;
        }

        number2_counts.insert(cur, current_count);
        i += 1;
    }

    // for number in number2_counts {
    //     println!("number2: {} {}", number.0, number.1);
    // }  

    let mut score = 0;
    for i in 0..number1.len() {
        let cur = number1[i];

        match number2_counts.get(&cur) {
            Some(count) => score += (&cur * count),
            None => score += 0
        }
    }

    println!("simularity score: {score}");

    Ok(())
}
