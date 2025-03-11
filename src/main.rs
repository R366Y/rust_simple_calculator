use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                // Parse number
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(n) = number.parse::<f64>() {
                    tokens.push(Token::Number(n));
                } else {
                    eprintln!("Invalid number: {}", number);
                }
            }
            '+' | '-' | '*' | '/' | '^' => {
                tokens.push(Token::Operator(c));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ' ' | '\t' => {
                // Skip whitespace
                chars.next();
            }
            _ => {
                eprintln!("Invalid character: {}", c);
                chars.next();
            }
        }
    }
    tokens
}

/// Higher precedence get evaluated first
fn get_precedence(op: char) -> u32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn is_left_associative(op: char) -> bool {
    match op {
        '+' | '-' | '*' | '/' => true,
        '^' => false,
        _ => true,
    }
}

/// Shunting Yard Algorithm
fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut output_queue = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push(token),
            Token::Operator(op) => {
                let current_precedence = get_precedence(op);

                while let Some(Token::Operator(stack_op)) = operator_stack.last() {
                    let stack_precedence = get_precedence(*stack_op);

                    if (is_left_associative(op) && current_precedence <= stack_precedence)
                        || (!is_left_associative(op) && current_precedence < stack_precedence)
                    {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(Token::Operator(op));
            }
            Token::LeftParen => operator_stack.push(token),
            Token::RightParen => {
                while let Some(token) = operator_stack.last() {
                    if let Token::LeftParen = token {
                        break;
                    }
                    output_queue.push(operator_stack.pop().unwrap());
                }

                // Pop the left parenthesis
                if let Some(Token::LeftParen) = operator_stack.last() {
                    operator_stack.pop();
                } else {
                    eprintln!("Mismatched parenthesis");
                }
            }
        }
    }

    // Pop any remaining operators from the stack
    while let Some(token) = operator_stack.pop() {
        if let Token::LeftParen = token {
            eprintln!("Mismatched parenthesis");
            continue;
        }
        output_queue.push(token);
    }
    output_queue
}

fn evaluate_postfix(postfix: Vec<Token>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in postfix {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(format!("Not enough operands for operator {}", op));
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        a / b
                    },
                    '^' => a.powf(b),
                    _ => return Err(format!("Unknown operator {}", op)),
                };

                stack.push(result);
            },
            _=> return Err(format!("Unexpected token in postfix{:?}", token)),
        }
    }
    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }
    Ok(stack.pop().unwrap())
}

fn main() {
    println!("Rust calculator");
    println!("Enter expression to calculate (or 'quit' to exit)");

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }
        let tokens = tokenize(input);
        // Debugging
        //println!("Tokens: {:?}", tokens);

        let postfix = infix_to_postfix(tokens);
        // Debugging
        // println!("Postfix: {:?}", postfix);

        match evaluate_postfix(postfix) {
            Ok(result) => println!("res = {}", result),
            Err(msg) => eprintln!("Error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests_tokens {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            format!("{:?}", tokenize("2 + 2")),
            format!(
                "{:?}",
                vec![Token::Number(2.0), Token::Operator('+'), Token::Number(2.0)]
            )
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            format!("{:?}", tokenize("")),
            format!("{:?}", Vec::<Token>::new())
        );
    }

    #[test]
    fn infix_to_postfix_test() {
        let tokens = tokenize("2 + 2");
        assert_eq!(
            format!("{:?}", infix_to_postfix(tokens)),
            format!(
                "{:?}",
                vec![Token::Number(2.0), Token::Number(2.0), Token::Operator('+')]
            )
        )
    }

    #[test]
    fn infix_to_postfix_test_with_precedence() {
        let tokens = tokenize("2 + 2 * 3");
        assert_eq!(
            format!("{:?}", infix_to_postfix(tokens)),
            format!(
                "{:?}",
                vec![
                    Token::Number(2.0),
                    Token::Number(2.0),
                    Token::Number(3.0),
                    Token::Operator('*'),
                    Token::Operator('+')
                ]
            )
        )
    }

    #[test]
    fn infix_to_postfix_test_with_parenthesis() {
        let tokens = tokenize("(2 + 2) * 3");
        assert_eq!(
            format!("{:?}", infix_to_postfix(tokens)),
            format!(
                "{:?}",
                vec![
                    Token::Number(2.0),
                    Token::Number(2.0),
                    Token::Operator('+'),
                    Token::Number(3.0),
                    Token::Operator('*')
                ]
            )
        )
    }
}
