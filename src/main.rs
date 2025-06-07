use std::{
    io::{Write, stdin, stdout},
    process,
};

mod commands;
mod interpreter;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Banner
    println!(r#"    ____              __          ____  _____"#);
    println!(r#"   / __ )____  __  __/ /_  ____ _/ __ \/ ___/"#);
    println!(r#"  / __  / __ \/ / / / __ \/ __ `/ / / /\__ \ "#);
    println!(r#" / /_/ / /_/ / /_/ / /_/ / /_/ / /_/ /___/ / "#);
    println!(r#"/_____/\____/\__,_/_.___/\__,_/\____//____/  "#);
    println!("====================================================");
    println!("Version 1.0 created June 2025");
    println!("Type 'help' to get a list of commands\n");

    let prompt = "$";
    let mut user_input = String::new();
    let mut error_code;

    loop {
        print!("{} ", prompt);
        stdout().flush()?;
        stdin().read_line(&mut user_input)?;
        error_code = parse_input(&user_input);

        if error_code == -1 {
            process::exit(99);
        }

        user_input.clear();
    }
}

fn parse_input(user_input: &str) -> i32 {
    let mut error_code = 0;

    for command in user_input.split(";") {
        let command = command.trim();
        let mut tokens = Vec::new();

        for token in command.split(" ") {
            if token != "" {
                tokens.push(token);
            }
        }

        let tokens_count = tokens.len();
        error_code = interpreter::interpret(tokens, tokens_count);
    }

    error_code
}
