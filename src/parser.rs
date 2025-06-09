use crate::interpreter;

pub fn parse_input(user_input: &str) -> anyhow::Result<()> {
    for command in user_input.split(";") {
        let command = command.trim();
        let mut tokens = Vec::new();

        for token in command.split(" ") {
            if token != "" {
                tokens.push(token);
            }
        }

        let tokens_count = tokens.len();
        interpreter::interpret(tokens, tokens_count)?;
    }

    // Success
    Ok(())
}
