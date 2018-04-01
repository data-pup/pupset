use config::Config;

/// Collect the arguments given to the program.
pub fn collect_args() -> Config {
    let args: Vec<String> = ::std::env::args()
        .skip(1)
        .collect();
    Config::new(args)
}

/// Print the arguments given to the program.
pub fn print_args(args: &Config) {
    println!("Received Arguments:");
    let mut i = 0;
    for a in args.argv.iter() {
        println!("[{}] : {}", i, *a);
        i += 1;
    }
}
