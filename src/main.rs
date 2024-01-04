use std::fs;

fn main() {
    let input = read_input().unwrap();
    println!("{input}");
}

fn read_input() -> std::io::Result<String> {
    fs::read_to_string("./src/input")
}
