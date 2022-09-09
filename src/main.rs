use std::{fs};
fn main() {
    let file_path="C:\\Users\\octon\\Documents\\git-cloned\\spellchecker\\src\\words.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut dict: Vec<&str>=Vec::new();
    for byte in contents.split_whitespace() {
        dict.push(byte);
    }
    for x in &dict {
        println!("{x}");
    }
}
