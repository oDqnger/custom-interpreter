use std::fs;
use std::env;

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
    
    for c in code.chars() {
         
    }

    println!("{}", code);
}

