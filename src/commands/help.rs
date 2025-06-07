use super::super::utils;

pub fn help(args_count: usize) -> i32 {
    if args_count > 1 {
        return utils::too_many_tokens();
    }

    let help_string = r#"COMMAND          DESCRIPTION
====================================================
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
