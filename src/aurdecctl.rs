use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg = args.get(1);

    if first_arg.map(|s| s.as_str()) == Some("--version") {
        println!("Hello, world!");
    }
}