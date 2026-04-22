use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (index, arg) in args.iter().enumerate() {
        println!("arg[{index}]: {arg}");
    }
}
