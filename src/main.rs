mod args;
mod config;
mod lib;

use args::{collect_args, print_args};
use config::Config;
use lib::{print_stdin_contents, handle_string_error};

fn main() {
    let args: Config = collect_args();
    print_args(&args);
    print_stdin_contents().unwrap_or_else(|err| handle_string_error(err));
}
