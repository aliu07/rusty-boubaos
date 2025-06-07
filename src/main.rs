use std::{
    io::{Write, stdin, stdout},
    process,
};

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
        error_code = interpreter(tokens, tokens_count);
    }

    error_code
}

fn interpreter(args: Vec<&str>, args_count: usize) -> i32 {
    let error_code;

    if args_count < 1 {
        badcommand();
    }

    match args[0] {
        "help" => error_code = help(),
        _ => error_code = badcommand(),
    }

    error_code
}

fn badcommand() -> i32 {
    println!("Unknown command");
    1
}

fn help() -> i32 {
    let help_string = r#"COMMAND          DESCRIPTION
================================================================
help             Displays all the commands
quit             Exits / terminates the shell with "Bye!"
set VAR STRING   Assigns a value to shell memory
print VAR        Displays the STRING assigned to VAR
run SCRIPT.TXT   Executes the file SCRIPT.TXT
echo STRING      Displays STRING. If STRING starts with $, prints
                 the value of the variable
my_ls            Lists the contents of the current directory
my_mkdir DIR     Creates a new directory with name DIR
my_touch FILE    Creates a new empty file with name FILE
my_cd DIR        Changes current directory to DIR
my_fork CMD      Executes CMD with ARGS using fork-exec pattern"#;

    println!("\n{}\n", help_string);
    0
}
