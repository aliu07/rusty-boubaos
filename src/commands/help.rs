pub fn help() {
    let help_string = r#"COMMAND          DESCRIPTION
====================================================
cd PATH          Changes directory to the provided PATH. Cannot
                 exit root directory.
echo STRING      Displays STRING. If STRING starts with $, prints
                 the value of the variable.
help             Displays all the commands.
ls               Displays all the directory entries.
mkdir DIR        Creates a directory called DIR.
print VAR        Displays the STRING assigned to VAR
pwd              Prints the current working directory path.
quit             Exits / terminates the shell with "Bye!".
remove VAR       Removes VAR from shell memory
rm FILE          Removes file named FILE from current directory.
rmdir DIR        Removes directory named DIR from current
                 directory.
run SCRIPT.TXT   Executes the text file SCRIPT.TXT
set VAR STRING   Assigns a value to shell memory
touch FILE       Creates a file named FILE."#;

    println!("\n{}\n", help_string);
}
