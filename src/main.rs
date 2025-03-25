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

        
        // Isolate the command ny separating at white space
       let command = get_command(input.clone());
       let mut arg = get_arg(input.clone());
        // Switch for commands
        match command.trim() {
            "echo" => {
                arg = echo(arg);
            },
            // Exit command
            "exit" => break,
            _ =>
            { // Invalid command
            println!("{}: command not found", input.trim())
            },

        };

        

    }
    
}

fn echo(s: String) -> String {
    println!("{s}");
    s
}

fn get_command(s: String) -> String{
    let command = {
    let mut c = String::from("");
    for i in s.chars() {
        if i != ' '{
            c.push(i);
        }else{
            break;
        }
    }
    c
    };
    command

}

fn get_arg(s: String) -> String{
    let mut c_over = false;

    let arg =
    {
        let mut a = String::from("");
        for i in s.chars(){
            if c_over {
                a.push(i);
            
            }
        if i == ' '{
            c_over = true;
        }
        
        }
        a
    };
    arg


}
