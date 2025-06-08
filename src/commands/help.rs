pub fn help() {
    let help_string = r#"COMMAND          DESCRIPTION
====================================================
echo STRING      Displays STRING. If STRING starts with $, prints
                 the value of the variable
help             Displays all the commands
print VAR        Displays the STRING assigned to VAR
pwd              Prints the current working directory path
quit             Exits / terminates the shell with "Bye!"
remove VAR       Removes VAR from shell memory
run SCRIPT.TXT   Executes the text file SCRIPT.TXT
set VAR STRING   Assigns a value to shell memory"#;

    println!("\n{}\n", help_string);
}
