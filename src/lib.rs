mod parser;
mod entity;

use crate::entity::Config;

fn print_help() {
    println!("Usage: rust_grep [options...]");
    println!("-f | --file <file_path>\tRead from file by <file_path>");
    println!("-h | --help\t\tGet help");
}

pub fn on_args(args: &[String]) {
    let result = parser::parse_args(args);
    match result {
        Ok(config) => {
            match config {
                Config::Help => print_help(),
                Config::File { content } => {
                    todo!("file:\n{content}")
                }
            }
        }
        Err(message) => {
            println!("Error: {message}");
            println!();
            print_help()
        }
    }
}
