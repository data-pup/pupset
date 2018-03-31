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

fn print_stdin_contents() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut handle = stdin.lock();
    match handle.read_line(&mut buf) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", buf);
        },
        Err(error) => println!("Error: {}", error),
    }
}

fn main() {
    let args = collect_args();
    print_args(&args);
    println!("Reading stdin:");
    print_stdin_contents();
}
