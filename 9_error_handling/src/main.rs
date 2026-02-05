use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[9];
    
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem when creating file: {e:#?}"),
            },
            _ => panic!("Problem when opening file: {error:#?}"),
        }
    };

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("hello_again should be open");

    Ok(())
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username_string = String::new();
    match username_file.read_to_string(&mut username_string) {
        Ok(_) => Ok(username_string),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username_string = String::new();
    username_file.read_to_string(&mut username_string)?;
    Ok(username_string)
}

fn last_char_on_the_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

