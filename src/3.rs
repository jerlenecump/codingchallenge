use std::fs;

fn main() {
    let mut file = fs::File::create("output.txt").unwrap();
    let data = "Hello, world!";
    file.write_all(data.as_bytes()).unwrap();
}
