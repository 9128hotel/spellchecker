use std::{fs, io::{stdin, stdout, Write}, mem};
fn main() {
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
    //start
    let contents=fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut dict: Vec<&str>=Vec::new();
    for byte in contents.split_whitespace() {
        dict.push(byte);
    }
    let index = dict.iter().position(|&a| a == check.to_owned()); //crashes if word doesn't exist

    if index == None {
        return;
    }

    for x in &dict {
        println!("{x}");
    }
    println!("{:?}", index);
    println!("{:?}", check);
    println!("{:?}", dict[index.unwrap()]);
}