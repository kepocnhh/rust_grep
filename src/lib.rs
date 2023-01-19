fn print_help() {
    println!("Usage: rust_grep [options...]");
    println!("-f <file_path>\tRead from file by <file_path>");
    println!("--help\t\tGet help");
}

fn on_file_path(file_path: &String) {
    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    todo!("contents:\n{contents}");
}

pub fn on_args(args: &[String]) {
    let len = args.len();

    if len == 1 {
        if args[0] == "--help" {
            print_help();
            return;
        }
    }

    if len == 2 {
        let flag = args[0].as_str();
        match flag {
            "-f" => on_file_path(&args[1]),
            _ => panic!("Flag \"{flag}\" is not supported!")
        }
    }

    print_help();
}
