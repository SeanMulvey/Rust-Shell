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


        // Switch for commands
        match input.trim() {
            // Exit command
            "exit 0" => break,
            _ =>
            { // Invalid command
            println!("{}: command not found", input.trim())
            },

        };

        

    }
    
}
