use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut equations = Vec::new();

    parse_input(&contents, &mut equations);

    let mut total_valid_result: i64 = 0;
    for i in 0..equations.len() {
        total_valid_result += analyze_solvable(&equations[i]);
    }

    println!("Total solvable calibration result: {total_valid_result}");

    Ok(())
}

struct Equation {
    result: i64,
    operands: Vec<i64>,
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Multiply,
    Concat,
}

fn parse_input(contents: &str, equations: &mut Vec<Equation>) {
    let mut lines = contents.lines();
    let mut done = false;
    while !done {
        match lines.next() {
            Some(line) => equations.push(create_equation(line)),
            None => done = true,
        }
    }
}

fn create_equation(line: &str) -> Equation {
    let colon_index = line.find(":").unwrap();

    let result = line[0..colon_index].parse::<i64>().unwrap();
    let operands_string = &line[colon_index + 1..];
    let operands: Vec<i64> = operands_string
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Equation { result, operands }
}

fn analyze_solvable(eq: &Equation) -> i64 {
    if is_solvable(eq) {
        return eq.result;
    } else {
        return 0;
    }
}

fn is_solvable(eq: &Equation) -> bool {
    let num_operators = eq.operands.len() - 1;

    let operators = generate_permutation(num_operators);

    for current_operator in operators {
        let mut current_result: i64 = eq.operands[0];
        for i in 0..current_operator.len() {
            current_result = match current_operator[i] {
                Operator::Plus => current_result + eq.operands[i + 1],
                Operator::Multiply => current_result * eq.operands[i + 1],
                Operator::Concat => {
                    let current_string = current_result.to_string();
                    let next_string = eq.operands[i + 1].to_string();
                    format!("{current_string}{next_string}")
                        .parse::<i64>()
                        .unwrap()
                }
            };
        }

        if current_result == eq.result {
            return true;
        }
    }
    return false;
}

fn generate_permutation(length: usize) -> Vec<Vec<Operator>> {
    let mut operators = Vec::new();

    let mut current = vec![Operator::Plus; length];

    generate_permutation_recursive(&mut current, 0, &mut operators);

    return operators;
}

fn generate_permutation_recursive(
    current: &mut Vec<Operator>,
    position: usize,
    operators: &mut Vec<Vec<Operator>>,
) {
    if position == current.len() {
        operators.push(current.clone());
        return;
    }

    current[position] = Operator::Plus;
    generate_permutation_recursive(current, position + 1, operators);

    current[position] = Operator::Multiply;
    generate_permutation_recursive(current, position + 1, operators);

    current[position] = Operator::Concat;
    generate_permutation_recursive(current, position + 1, operators);
}
