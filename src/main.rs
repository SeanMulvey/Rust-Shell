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
       let arg = get_arg(input.clone());
        // Switch for commands
        match command.trim() {
            "echo" => {
                echo(arg);
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
    let mut cOver = false;

    let arg =
    {
        let mut a = String::from("");
        for i in s.chars(){
            if cOver {
                a.push(i);
            
            }
        if i == ' '{
            cOver = true;
        }
        
        }
        a
    };
    arg


}
