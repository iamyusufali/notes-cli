use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("add") => println!("Adding note..."),
        Some("list") => println!("Listing notes..."),
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: notes <command>");
    println!("Commands:");
    println!("  add   Add a note");
    println!("  list  List notes");
}
