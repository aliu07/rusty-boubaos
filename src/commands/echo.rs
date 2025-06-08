use crate::{commands, errors::BadCommandError};

pub fn echo(variable: &str) -> Result<(), BadCommandError> {
    if variable.chars().nth(0).unwrap() == '$' {
        // Get rid of dollar sign
        let variable = &variable[1..];

        commands::print(variable)?
    } else {
        println!("{}", variable);
    }

    Ok(())
}
