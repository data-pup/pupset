# Development Notes

NOTE: This program was written following advice using the Rust book.

## Parsing command line arguments

```rust
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
```

The first task was adding some simple logic to collect the arguments given to
the program. This will be expounded on further, but for now we are concerned
with making sure that we can identify the arguments to the program.

## Print stdin to stdout

Before we implement any of the stream editing functionality, we should first
make sure that we can read input from the standard input, and print the
output, without making any changes. To do that, we will need to figure out how
to read input, which will involve learning more about the `std::io` crate.

### Reading standard input

In order to read from stdin, we will need to use the `std::io::stdin()` method
in order to obtain a handle to the process's standard input. Then, we lock it,
as access to this buffer is synchronized via a mutex. We must also import the
`BufRead` trait, in order to use the `read_line` method. The resulting code
looks like this:

```rust
use std::io;
use std::io::BufRead;

fn print_stdin_contents() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        match line {
            Ok(text) => println!("{}", text),
            Err(e) => {
                println!("Error {}", e);
                return;
            },
        };
    }
}

fn main() {
    print_stdin_contents().unwrap_or_else(|err| {
        eprintln!("Error reading input: {}", err);
        std::process::exit(1);
    });
}
```

This is a basic example of reading standard input, and we will work on this
further as we progress. Eventually, it would be nice to specify input files,
output files, but we will focus on some basic commands first.
