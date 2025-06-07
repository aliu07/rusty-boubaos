use super::commands;
use super::utils;

const MAX_ARGS_COUNT: usize = 7;

pub fn interpret(args: Vec<&str>, args_count: usize) -> i32 {
    let error_code;

    if args_count < 1 {
        return utils::missing_args();
    }

    if args_count > MAX_ARGS_COUNT {
        return utils::too_many_tokens();
    }

    if args[0] == "quit" {
        commands::quit();
    }

    match args[0] {
        "help" => error_code = commands::help(args_count),
        _ => error_code = utils::unknown_command(),
    }

    error_code
}
