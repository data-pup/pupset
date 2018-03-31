use std::io::BufRead;
use std::io::Error as IoError;

/// Read input from stdin and print it to stdout.
pub fn print_stdin_contents() -> Result<(), IoError> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    println!("Reading stdin:");
    for line_result in handle.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())
}

/// Print an I/O error if a problem occurred while reading from stdin.
pub fn handle_string_error(err: IoError) {
    eprintln!("Error reading input: {}", err);
}
