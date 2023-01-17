fn print_help() {
    println!("Usage: rust_grep [options...]");
    println!("-f <file_path>\tRead from file by <file_path>");
    println!("--help\t\tGet help");
}

fn on_file_path(file_path: &String) {
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    todo!("contents: {contents}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let len = args.len();
    if len < 2 {
        panic!("Args size error!");
    }

    if len == 2 {
        if args[1] == "--help" {
            print_help();
            return;
        }
    }

    if len == 3 {
        let flag = args[1].as_str();
        match flag {
            "-f" => on_file_path(&args[2]),
            _ => panic!("Flag \"{flag}\" is not supported!")
        }
    }

    print_help();
}
