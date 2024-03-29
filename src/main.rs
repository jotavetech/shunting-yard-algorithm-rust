use std::io;

fn main() {
    let mut expression = String::new();

    println!("Enter your expression: ");

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");

    let postfix = infix_to_postfix(&expression);

    let result = evaluate_postfix(postfix).pop().unwrap();

    println!("Result of your expression is {}", result);
}

fn evaluate_postfix(expression: Vec<&str>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for token in expression {
        if let Ok(number) = token.parse::<i32>() {
            stack.push(number);
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Operador inválido: {}", token),
            };

            stack.push(result);
        }
    }

    if stack.len() != 1 {
        panic!("Operação inválida");
    }

    stack
}

fn infix_to_postfix(expression: &String) -> Vec<&str> {
    let mut output: Vec<&str> = Vec::new();
    let mut stack: Vec<&str> = Vec::new();
    let tokens = tokenize(expression);

    for token in tokens {
        if token.chars().all(char::is_numeric) {
            output.push(token);
            println!("OUTPUT: {:?}", output);
        } else if is_operator(token) {
            while let Some(operator) = stack.last() {
                if get_precedence(operator) >= get_precedence(token) {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(token);
            println!("STACK: {:?}", stack);
        }
    }

    while let Some(operator) = stack.pop() {
        output.push(operator);
    }

    output
}

// utils functions

fn get_precedence(char: &str) -> i32 {
    match char {
        "+" | "-" => 1,
        "/" | "*" => 2,
        _ => 0,
    }
}

fn is_operator(char: &str) -> bool {
    match char {
        "+" | "-" | "*" | "/" => true,
        _ => false,
    }
}

fn tokenize(token: &str) -> Vec<&str> {
    let new_token: Vec<&str> = token.split_whitespace().collect();
    new_token
}
