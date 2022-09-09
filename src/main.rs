use std::{fs, borrow::Borrow};
fn main() {
    let file_path="/home/lucy/code/micro/src/words.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let dict: Vec<String>;
    let mut op=0;
    for byte in contents.split_whitespace() {
        &dict[op] = byte.to_string().borrow();
        op+=1;
    }
}
