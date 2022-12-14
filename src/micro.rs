use std::{fs::{self, File}, io::{stdin, stdout, Write, Read}};

fn tui(code:i8, check:String, dict_path:&str, mut file:File, dict: Vec<&str>) {
    let mut looper=true;
    if code == 2 {
        let mut contents:String = String::new();
        file.read_to_string(&mut contents);
        while looper==true {
            println!("{check} is not in the dictionary.");
            let mut inp=String::new();
    
            print!("[F]ix     [A]dd to dictionary");
            let _=stdout().flush();
            stdin().read_line(&mut inp).expect("Did not enter a correct string");
            if let Some('\n')=inp.chars().next_back() {
                inp.pop();
            }
            if let Some('\r')=inp.chars().next_back() {
                inp.pop();
            }
    
            if inp == "F" || inp == "f" {
                looper=false;
                let mut sim:Vec<i32>=vec![];
                for x in 0..dict.len() {
                    let check_vec:Vec<char> = check.chars().collect();
                    let dict_vec:Vec<char> = dict[x].chars().collect();

                    if check_vec.len() > dict_vec.len() {
                        for y in 0..dict_vec.len() {
                            if dict_vec[y] == check_vec[y] {
                                sim[y]+=1
                            }
                        }
                    }
                }
                
            }
            else if inp=="A" || inp=="a" {
                looper=false
            }
        }
    }
}

pub fn spellcheck(check:String, dict_path:&str, file:File) {
    let mut code: i8 = 0;
    let contents=fs::read_to_string(dict_path).expect("Should have been able to read the file");
    let mut dict: Vec<&str>=Vec::new();
    for byte in contents.split_whitespace() {
        dict.push(byte);
    }
    let index = dict.iter().position(|&a| a == check.to_owned());

    if index == None {
        code = 2;
    }

    /*
    for x in &dict {
        println!("{x}");
    }
    */
    tui(code, check, dict_path, file, dict);
    
}
