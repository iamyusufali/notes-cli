use std::env;
use std::io::{self, Write};
use std::process;

// Read CLI args and decide which path to run.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str);

    match command {
        Some("add") => run_command(handle_add()),
        Some("list") => run_command(handle_list()),
        _ => print_help(),
    }
}

fn run_command(result: io::Result<()>) {
    if let Err(error) = result {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn handle_add() -> io::Result<()> {
    println!("Add a new note");

    let item_type = prompt_item_type()?;
    let category = prompt_non_empty("Enter category: ")?;
    let value = match item_type.as_str() {
        "text" => prompt_non_empty("Enter text content: ")?,
        "link" => prompt_non_empty("Enter link URL: ")?,
        _ => unreachable!("prompt_item_type only returns text or link"),
    };

    println!();
    println!("Captured note details:");
    println!("type: {item_type}");
    println!("category: {category}");

    match item_type.as_str() {
        "text" => println!("content: {value}"),
        "link" => println!("url: {value}"),
        _ => unreachable!("prompt_item_type only returns text or link"),
    }

    println!("Next step: save this note to storage.");

    Ok(())
}

fn handle_list() -> io::Result<()> {
    println!("List notes");

    let category = prompt_non_empty("Enter category to list: ")?;

    println!();
    println!("Selected category: {category}");
    println!("Next step: load notes from storage for this category.");

    Ok(())
}

fn prompt_item_type() -> io::Result<String> {
    loop {
        let input = prompt("Select type (text/link): ")?;
        let normalized = input.to_lowercase();

        match normalized.as_str() {
            "text" | "link" => return Ok(normalized),
            _ => println!("Invalid type. Please enter 'text' or 'link'."),
        }
    }
}

fn prompt_non_empty(message: &str) -> io::Result<String> {
    loop {
        let input = prompt(message)?;

        if input.is_empty() {
            println!("Input cannot be empty. Please try again.");
        } else {
            return Ok(input);
        }
    }
}

fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn print_help() {
    println!("Usage: notes <command>");
    println!("Commands:");
    println!("  add   Add a note");
    println!("  list  List notes");
}
