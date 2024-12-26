use std::{collections::HashMap, fs::File, i32, io::{self, Read}};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut page_order_rules = HashMap::new();

    let rule_index = contents.find("\r\n\r\n").unwrap();

    let mut rule_lines = contents[0..rule_index].lines();
    let mut done = false;
    while !done {
        match rule_lines.next() {
            Some(line) => add_rule_line(line, &mut page_order_rules),
            None => done = true
        }
    }

    let mut sum_middle_valid_pages = 0;
    let mut sum_middle_invalid_pages = 0;

    let print_update_contents = &contents[rule_index + 4..];
    let mut print_update_lines = print_update_contents.lines();
    done = false;
    while !done {
        match print_update_lines.next() {
            Some(line) => {
                let (valid, invalid) = analyze_update(line, &page_order_rules);
                sum_middle_valid_pages += valid;
                sum_middle_invalid_pages += invalid;
            }
            None => done = true
        }
    }

    println!("sum of middle valid pages: {sum_middle_valid_pages}");
    println!("sum of middle invalid pages: {sum_middle_invalid_pages}");

    Ok(())
}

fn add_rule_line(line: &str, rules: &mut HashMap::<i32, Vec<i32>>) {

    let separator_index = line.find('|').unwrap();

    let first_page = line[0..separator_index].parse::<i32>().unwrap();
    let second_page = line[separator_index+1..].parse::<i32>().unwrap();

    let later_pages = rules.entry(first_page).or_insert(Vec::new());
    later_pages.push(second_page);
}

// returns a tuple of the middle page values, (valid_middle_page, 0) or (0, fixed_invalid_middle_page)
fn analyze_update(line: &str, rules: &HashMap::<i32, Vec<i32>>) -> (i32, i32) {
    let pages: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

    let isvalid = check_valid(&pages, rules);

    if isvalid {
        let middle_page = pages[pages.len() / 2];
        return (middle_page, 0);
    }
    else {
        let new_pages = reorder_pages(&pages, rules);
        let middle_page = new_pages[new_pages.len() / 2];

        if !check_valid(&new_pages, rules) {
            println!("Invalid {line}");
        }

        return (0, middle_page);
    }
}

fn check_valid(pages: &Vec<i32>, rules: &HashMap::<i32, Vec<i32>>) -> bool {
    let mut isvalid = true;
    for i in 0..pages.len() {
        match rules.get(&pages[i]) {
            Some(later_pages) =>{
                for j in 0..i {
                    if later_pages.contains(&pages[j]) {
                        isvalid = false;
                    }
                }
            }
            None => ()
        }
    }
    return isvalid;
}

fn reorder_pages(pages: &Vec<i32>, rules: &HashMap::<i32, Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..pages.len() {
        visit(pages[i], pages, &mut result, rules);
    }
    return result;
}

fn visit(page: i32, pages: &Vec<i32>, result: &mut Vec<i32>, rules: &HashMap::<i32, Vec<i32>>) {
    if result.contains(&page) {
        return;
    }

    match rules.get(&page) {
        Some(later_pages) =>{
            for later_page in later_pages {
                if pages.contains(&later_page) {
                    visit(*later_page, pages, result, rules);
                }
            }
        }
        None => ()
    }

    result.insert(0, page);
}