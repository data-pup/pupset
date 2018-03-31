use std::io;
use std::io::BufRead;
use std::env;

/// Collect the arguments given to the program.
fn collect_args() -> Vec<String> {
    let args = env::args()
        .skip(1)
        .collect();
    args
}

/// Print the arguments given to the program.
fn print_args(args: &Vec<String>) {
    println!("Received Arguments:");
    let mut i = 0;
    for a in args.iter() {
        println!("[{}] : {}", i, *a);
        i += 1;
    }
}

/// Read input from stdin and print it to stdout.
fn print_stdin_contents() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    println!("Reading stdin:");
    for line_result in handle.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())
}

/// Print an I/O error if a problem occurred while reading from stdin.
fn handle_string_error(err: io::Error) {
    eprintln!("Error reading input: {}", err);
}

fn main() {
    let args: Vec<String> = collect_args();
    print_args(&args);
    print_stdin_contents().unwrap_or_else(|err| handle_string_error(err));
}
