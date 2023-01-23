mod parser;
mod entity;

use crate::entity::Config;

fn print_help() {
    println!("Usage: rust_grep [options...]");
    println!("{:?} <file_path>\tRead from file by <file_path>", Config::File.get_flags());
    println!("{:?}\t\tGet help", Config::Help.get_flags());
}

pub fn on_args(args: &[String]) {
    let result = parser::parse_args(args);
    match result {
        Ok(config) => {
            match config {
                Config::Help => print_help(),
                Config::File => {
                    todo!("file!")
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
