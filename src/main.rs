
use crate::builtins::COMMANDS;
use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};


fn main() {


    // Enable REPL (Read-Eval-Print-Loop)
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut path = String::from("");
        let tokens = tokenize(&input);
        println!("{}", &tokens[0]);


        if tokens.len() > 0{
                // Exit if command is exit
            if &tokens[0] == "exit"{
                break;
            }else{
                commands(&tokens[0], &input);
            }

        }
        else {
            
            if tokens[0].trim().starts_with("PATH="){
                path = tokens[0][5..].to_string();
                println!("{}", path);
            } else{
                invalid(&input);
            }
            
        }

        if &tokens[0].to_string()[..4] == "PATH="{
            path = tokens[0][5..].to_string();
            println!("{}", path);
        }
        
        

        


        
    }
    
}

// Separate String into tokens based on whitespace
fn tokenize(s: &String) -> Vec<String>{
    let mut tokens = Vec::new();
    let mut token = String::from("");

    for c in s.chars(){
        if c != ' '{
            token.push(c);
        }else{
            tokens.push(token);
            token = String::from("");
        }
    }
    tokens

}

// Command cases
fn commands(t: &String, i: &String){
    match t.as_str(){
        
        "type" => {
            get_type(&i.to_string());
        }
        "echo" => {
            echo(i);
        },
        _ => {
            invalid(t);
        }
    }

}

// Call when command is invalid
fn invalid(c: &String){
    println!("{}: command not found", c.trim());
}

// Print arg after echo command
fn echo(s: &str) {
    println!("{}",&s.trim()[5..]);
}

// Type switch
fn get_type(s: &String){
    match &s.trim()[5..]{
        // shell builtins
        "type" | "echo" | "exit" =>{
            println!("{} is a shell builtin",&s.trim()[5..]);
        },
        _ => {
            println!("{}: not found",&s[5..].to_string().trim());
        }
    };
}


fn set_path(s: &String) -> String {
    s.to_string()
}





