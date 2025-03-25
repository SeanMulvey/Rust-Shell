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

        // Switch for cli input
        match input.trim(){
            // type case
            input if input.starts_with("type ") => {
                match &input[5..] {
                    // echo
                    "echo" => {
                        println!("echo is a shell builtin");
                    },
                    // exit
                    "exit" =>
                    {
                        println!("exit is a shell builtin");
                    },
                    //Default
                    _ => {
                        println!("{}: not found", &input[5..] );
                    },
                };
            }
            // echo case
            input if input.starts_with("echo ") => {
                println!("{}", &input[5..]);
            },
            // exit case
            "exit 0" => break,
            // Default
            _ => {
                println!("{}: command not found", input.trim());
            },
        };
    }
    
}






