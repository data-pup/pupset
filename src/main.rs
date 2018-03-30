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

fn main() {
    let args = collect_args();
    print_args(&args);
}
