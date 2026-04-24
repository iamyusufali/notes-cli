use std::env;

// Read CLI args and decide which path to run.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("add") => handle_add(),
        Some("list") => handle_list(),
        _ => print_help(),
    }
}

fn handle_add() {
    println!("Adding note...");
}

fn handle_list() {
    println!("Listing notes...");
}

fn print_help() {
    println!("Usage: notes <command>");
    println!("Commands:");
    println!("  add   Add a note");
    println!("  list  List notes");
}
