//! # EasyDoc
//!
//! `EasyDoc` is a collection of utilities to make performing certain
//! Documentations more convenient.

/// Adds one to the number given.


/// Simple CLI application
use std::fs;
use std::io;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    filename: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Optional message to add
    #[arg(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        print!("Filename = {}!", args.filename);
        if let Some(msg) = &args.message{
            print!(" {}", msg);
        }
        println!();
    }

    let content= read_file(&args.filename).unwrap_or_else(|error| {
        eprintln!("Error reading file: {}", error);
        String::new()
    });
    
    println!("{}", content);
    
    
}
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path).map_err(|err| {
        eprintln!("Failed to read {}: {}", path, err);
        err
    })
}
