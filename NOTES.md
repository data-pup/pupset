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

...
