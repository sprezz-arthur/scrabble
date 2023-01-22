fn ask_name() -> String {
    use std::io;

    println!("What is your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("failed to readline");

    return name.trim().to_string();
}

fn main() {
    let name: String = ask_name();
    println!("Hello, {}!", name.trim());
}
