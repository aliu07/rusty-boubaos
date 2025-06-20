use std::io::{BufRead, Write, stdin, stdout};

use crate::environment::ENV;
use crate::memory::VARIABLE_TABLE;

mod commands;
mod environment;
mod errors;
mod interpreter;
mod memory;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // True = Interactive mode, False = Batch mode (input piped)
    let mode_flag = atty::is(atty::Stream::Stdin);
    // Init
    let _env = &ENV;
    let _var_table = &VARIABLE_TABLE;

    if mode_flag {
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

        loop {
            print!("{} ", prompt);
            stdout().flush()?;
            stdin().read_line(&mut user_input)?;
            parser::parse_input(&user_input).unwrap_or_else(|err| {
                eprintln!("{}", err);
            });

            user_input.clear();
        }
    } else {
        for line in stdin().lock().lines() {
            let line = line?;

            parser::parse_input(&line).unwrap_or_else(|err| {
                eprintln!("{}", err);
            });
        }

        Ok(())
    }
}
