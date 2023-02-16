use std::io;
use std::io::prelude::*;

pub fn readline(prompt: String)-> String{
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_goes_into_input_above)=>{},
        Err(_no_updates_is_fine)=>{},
    }
    let line: String = input.trim().to_string();
    line
}

pub fn clear()-> (){
    std::process::Command::new("clear").status().unwrap();
}

pub fn pause(){
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn read_number()-> i32{
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");


    let trimmed = number.trim();
    let aint: i32 = trimmed.trim().parse().ok().expect("Error in user input");
    match trimmed.parse ::<i32>(){
        Ok(_i)=> println!(""),
        Err(..)=> println!("This was not an integer: {}", number),
    };
    return aint;
}


