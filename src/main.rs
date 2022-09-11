use std::{io::{stdin, stdout, Write}, fs::{File}};
mod micro;
fn main() {
    let file = File::open("foo.txt").unwrap();
    let file_path="/home/lucy/code/spellchecker/src/words.txt";
    let mut check=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut check).expect("Did not enter a correct string");
    if let Some('\n')=check.chars().next_back() {
        check.pop();
    }
    if let Some('\r')=check.chars().next_back() {
        check.pop();
    }

    micro::spellcheck(check, file_path, file);
}
