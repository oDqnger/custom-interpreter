use std::fs;
use std::env;

#[derive(Debug)]

enum TokenTypes {
    Integer,
    PlusSign,
    MinusSign,
    MultiplySign,
    DivisionSign,
    EOL,
}

struct Lexer {
    token: TokenTypes,
    value: String,
}

impl Lexer {
    fn identify_token(c: char) -> Result<Self, String> {
        if c.is_digit(10) {
            Ok(Self {
                token: TokenTypes::Integer,
                value: c.to_string(),
            })
        } else if c == '+' {
            Ok(Self {
                token: TokenTypes::PlusSign,
                value: c.to_string(),
            })
        } else if c == '-' {
            Ok(Self {
                token: TokenTypes::MinusSign,
                value: c.to_string(),
            })
        } else if c == '*' {
            Ok(Self {
                token: TokenTypes::MultiplySign,
                value: c.to_string(),
            })
        } else if c == '/' {
            Ok(Self {
                token: TokenTypes::DivisionSign,
                value: c.to_string(),
            })
        } else if c == ';' {
            Ok(Self {
                token: TokenTypes::EOL,
                value: c.to_string(),
            })
        } else {
            Err("I DONT UDNERSTAND".to_string())
        }
    }
}

fn remove_whitespace(code: String) -> String {
    let mut formatted_code = String::new();
    for ch in code.chars() {
        if !ch.is_whitespace() {
            formatted_code.push(ch);
        }
    }

    formatted_code
}

fn read_file() -> String {
    let args : Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents
}

fn main() { 
    let code = remove_whitespace(read_file());
    let mut lexer;
    for c in code.chars() {
        lexer = Lexer::identify_token(c).unwrap();
        println!("{:?}", lexer.token);
    }

    println!("{}", code);
}

