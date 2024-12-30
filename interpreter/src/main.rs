use std::fs;
use std::env;
use std::mem;

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
            Err("Syntax error!".to_string())
        }
    }
}

struct Expr {
    token_stream: Vec<Lexer>,
    value: i32,
}

impl Expr {
    fn eat(&self, token_type: Vec<TokenTypes>) -> Result<bool, String> {
        for token_pos in 0..token_type.len() - 1 {
            if mem::discriminant(&token_type[token_pos]) == mem::discriminant(&(self.token_stream[token_pos].token)) {
                return Err("Syntax error!".to_string())
            }
        }
        
        Ok(true)
    }

    fn group_expressions(&mut self, lexer: Lexer) {
        if let TokenTypes::EOL = lexer.token {
            if self.eat(vec![TokenTypes::Integer, TokenTypes::PlusSign, TokenTypes::Integer]).unwrap() == true {
                self.value = self.token_stream[0].value.parse::<i32>().unwrap() + self.token_stream[2].value.parse::<i32>().unwrap();
                self.token_stream = vec![];
            }
        } else {
            self.token_stream.push(lexer);
        }
    }
}

//fn isdigit(s: &String) -> bool {
//    let mut isd = true;
//    for c in s.chars() {
//        if !c.is_digit(10) {
//            isd = false;
//            return isd
//        }
//    }
//
//    isd
//}

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
    let mut expr = Expr{ token_stream: vec![], value: 0 };
    for c in code.chars() {
        lexer = Lexer::identify_token(c).unwrap();
        //println!("{:?}", lexer.token);
        expr.group_expressions(lexer);
        //println!("{}", expr.value);
        //for x in expr.token_stream.iter() {
        //    println!("{:?}", x.token);
        //}
    }
}

