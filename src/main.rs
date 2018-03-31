use std::io;
use std::io::BufRead;
use std::env;

fn collect_args() -> Vec<String> {
    let args = env::args()
        .skip(1)
        .collect();
    args
}

fn print_args(args: &Vec<String>) {
    println!("Received Arguments:");
    let mut i = 0;
    for a in args.iter() {
        println!("[{}] : {}", i, *a);
        i += 1;
    }
}

fn print_stdin_contents() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line_result in handle.lines() {
        match line_result {
            Ok(line) => println!("{}", line),
            Err(err) => {
                return Err(err);
            },
        }
    }
    Ok(())
}

fn main() {
    let args = collect_args();
    print_args(&args);
    println!("Reading stdin:");
    print_stdin_contents().unwrap_or_else(|err| {
        eprintln!("Error reading input: {}", err);
        std::process::exit(1);
    });
}
