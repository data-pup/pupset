mod args;
mod lib;

use args::{collect_args, print_args};
use lib::{print_stdin_contents, handle_string_error};

fn main() {
    let args: Vec<String> = collect_args();
    print_args(&args);
    print_stdin_contents().unwrap_or_else(|err| handle_string_error(err));
}
