use std::io;
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

enum Token {
    Number (u32),
    Operation (&str),
}

fn tokenize(&str input) -> Vec<Token> {
    
}
