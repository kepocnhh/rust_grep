fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        println!("No args.");
        return;
    }

    println!("Args:");
    for (index, it) in args.iter().enumerate() {
        println!("{index}] {it}");
    }
}
