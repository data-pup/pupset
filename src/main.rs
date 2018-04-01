mod args;
mod command;
mod config;
mod lib;

use args::{collect_args, print_args};
use config::Config;
use lib::{run, handle_string_error};

fn main() {
    let args: Config = collect_args();
    print_args(&args);
    run().unwrap_or_else(|err| handle_string_error(err));
}
