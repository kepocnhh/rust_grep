fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Args size error!");
    }

    rust_grep::on_args(&args[1..]);
}
