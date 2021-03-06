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

### Refactoring

Next I cleaned up the function that prints the lines from stdin. This was a
nice chance to use to `?` operator.

```rust
for line_result in handle.lines() {
    let line = line_result?;
    println!("{}", line);
}
```

At this point, the function would do something like this when I ran it:

```sh
✨  cargo run hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/pupset hello world`
Received Arguments:
[0] : hello
[1] : world
Reading stdin:
foo                     // <- input
foo                     // <- output
bar
bar
```

This is a decent scaffold, and from here I can move into some further steps
on this project. Right now, the main priorities are:
*  Move the argv logic into a new module.
*  Add some logic to parse the arguments into commands.
*  Add some simple tests.
*  Add traits and classes for commands, start with a line numbering command.

## Option parsing

The next step will be refactoring some of the argument parsing code. Eventually
we will need to implement some different stream editing commands such as
printing line numbers, search and replace, and so forth. Before we can do that
however, we will need to first have a foundation for how we implement commands
generically.

This could be done using an option parsing crate, but we will try and implement
this using the standard library alone, because learning things is fun.

### Project Structure

At this point I determined that it would be best to start thinking about the
general shape of the code. The main runtime of the program itself should be
kept in `main.rs`. This should be very sparse however, and mostly be kept to
fetching arguments given to the program, and passing these to a `run` method
in the `lib.rs` file.

I decided to add a subdirectory named `command/`, which will end up storing
structs, traits, and definitions for different commands. I decided this
warranted its own subdirectory because commands will end up having address
components, regex patterns, and other various logical components.

Options regarding the program configuration itself should be considered
separately. Files related to this will be placed in a `config/` directory.
