pub fn help() {
    let help_string = r#"COMMAND          DESCRIPTION
====================================================
help             Displays all the commands
quit             Exits / terminates the shell with "Bye!"
set VAR STRING   Assigns a value to shell memory
remove VAR       Removes VAR from shell memory
print VAR        Displays the STRING assigned to VAR
run SCRIPT.TXT   Executes the file SCRIPT.TXT
echo STRING      Displays STRING. If STRING starts with $, prints
                 the value of the variable"#;

    println!("\n{}\n", help_string);
}
