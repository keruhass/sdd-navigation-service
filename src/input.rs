use std::io;

pub fn path_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    String::from(input.trim())
}
