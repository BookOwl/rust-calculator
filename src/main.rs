use std::io;
extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;

fn main() {
    loop {
        println!("Enter input:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let tokens = tokenize(input);
        let stack = shunt(tokens);
        let res = calculate(stack);
        println!("{}", res);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Number (i64),
    Plus,
    Sub,
    Mul,
    Div,
    LeftParen,
    RightParen,
}

/// Tokenizes the input string into a Vec of Tokens.
fn tokenize(mut input: String) -> Vec<Token> {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
    }
    let mut res = vec![];
    while !(input.trim_left().is_empty()) {
        input = input.trim_left().to_string();
        input = if let Some((_, end)) = NUMBER_RE.find(&input) {
            let (num, rest) = input.split_at_mut(end);
            res.push(Token::Number(num.parse::<i64>().unwrap()));
            rest.to_string()
        } else {
            res.push(match input.chars().nth(0) {
                Some('+') => Token::Plus,
                Some('-') => Token::Sub,
                Some('*') => Token::Mul,
                Some('/') => Token::Div,
                Some('(') => Token::LeftParen,
                Some(')') => Token::RightParen,
                _ => panic!("Unknown character!")
            });
            input.trim_left_matches(|c| c == '+' ||
                                        c == '-' ||
                                        c == '*' ||
                                        c == '/' ||
                                        c == '(' ||
                                        c == ')').to_string()
        }
    }
    res
}

/// Transforms the tokens created by `tokenize` into RPN using the
/// [Shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
fn shunt(tokens: Vec<Token>) -> Vec<Token> {
    let mut queue: Vec<Token> = vec![];
    let mut stack: Vec<Token> = vec![];
    for token in tokens {
        match token {
            n @ Token::Number(_) => queue.push(n),
            op @ Token::Plus | op @ Token::Sub |
            op @ Token::Mul  | op @ Token::Div => {
                while let Some(o) = stack.pop() {
                    if precedence(&op) <= precedence(&o) {
                        queue.push(o);
                    } else {
                        stack.push(o);
                        break;
                    }
                }
                stack.push(op)
            },
            p @ Token::LeftParen => stack.push(p),
            Token::RightParen => {
                let mut found_paren = false;
                while let Some(op) = stack.pop() {
                    match op {
                        Token::LeftParen => {
                            found_paren = true;
                            break;
                        },
                        _ => queue.push(op)
                    }
                }
                assert!(found_paren)
            }
        }
    }
    while let Some(op) = stack.pop() {
        queue.push(op);
    }
    queue
}

/// Takes a Vec of Tokens converted to RPN by `shunt` and calculates the result
fn calculate(tokens: Vec<Token>) -> i64 {
    let mut stack = vec![];
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Plus => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a + b);
            },
            Token::Sub => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a - b);
            },
            Token::Mul => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a * b);
            },
            Token::Div => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(a / b);
            },
            _ => unreachable!() // By the time the token stream gets here, all the LeftParen
                                // and RightParen tokens will have been removed by shunt()
        }
    }
    stack[0]
}

/// Returns the precedence of op
fn precedence(op: &Token) -> usize {
    match op {
        &Token::Plus | &Token::Sub => 1,
        &Token::Mul  | &Token::Div => 2,
        _ => 0,
    }
}
