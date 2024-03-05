use std::{env, io::Write, process::exit};

mod todo_funcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("input something");
        exit(1);
    }
    let command: &String = &args[1];
    match command.as_str() {
        "add" => {
            let mut input: String = String::new();
            print!("Enter the goal: ");
            std::io::stdout().flush().expect("failed to flush buffer");
            std::io::stdin().read_line(&mut input).expect("failed to read input");
            match todo_funcs::add(&input) {
                Ok(_) => println!("Successfully added to todo!"),
                Err(e) => eprintln!("Failed to add to todo: {}", e)
            }
        },
        "remove" => {
            todo_funcs::remove()
        },
        "list" => {
            todo_funcs::list()
        },
        _ => println!("invalid command"),
    }
}
