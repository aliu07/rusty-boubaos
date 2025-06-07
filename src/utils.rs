pub fn unknown_command() -> i32 {
    println!("Bad command: Unknown command");
    1
}

pub fn missing_args() -> i32 {
    println!("Bad command: Missing arguments");
    2
}

pub fn too_many_tokens() -> i32 {
    println!("Bad command: Too many tokens");
    3
}
