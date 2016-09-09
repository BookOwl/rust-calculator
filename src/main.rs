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
        println!("{:?}", tokens);
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
                _ => panic!("Unknown character!")
            });
            input.trim_left_matches(|c| c == '+' ||
                                        c == '-' ||
                                        c == '*' ||
                                        c == '/').to_string()
        }
    }
    res
}

/// Transforms the tokens created by `tokenize` into RPN using the
/// [Shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
fn shunt(tokens: Vec<Token>) -> Vec<Token> {
    vec![Token::Number(0)]
}

/// Takes a Vec of Tokens converted to RPN by `shunt` and calculates the result
fn calculate(stack: Vec<Token>) -> i64 {
    0
}
