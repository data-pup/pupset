/// Collect the arguments given to the program.
pub fn collect_args() -> Vec<String> {
    let args = std::env::args()
        .skip(1)
        .collect();
    args
}

/// Print the arguments given to the program.
pub fn print_args(args: &Vec<String>) {
    println!("Received Arguments:");
    let mut i = 0;
    for a in args.iter() {
        println!("[{}] : {}", i, *a);
        i += 1;
    }
}
