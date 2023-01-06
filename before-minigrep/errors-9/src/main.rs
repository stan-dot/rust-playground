use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let v = vec![1,2,3];
    // v[99];
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file)=>file,
        // Err(error)=>panic!("problem opening the file: {:?}", error),
        Err(error)=> match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=> panic!("problem creating the file: {:?}", e),
            },
            other_error =>{
                panic!("problem opening the file: {:?}", other_error)
            }
        },
    };

    test_lastchar();
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn test_lastchar() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );
    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

fn hardcoded_address_no_panic() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}


fn read_username_with_operator(filename:&str)-> Result<String, io::Error>{

    let mut username_file = File::open(filename);
    let mut username = String::new();
    username_file?.read_to_string(&mut username);
    // that is equivalent
    // File::open(filename)?.read_to_string(&mut username)?;
    // fs::read_to_string(filename);
    Ok(username)
}

fn read_username_from_file(filename:&str) ->Result<String, io::Error>{


    let username_file_result = File::open(filename);
    let mut username_file = match username_file_result {
        Ok(file)=>file,
        Err(e)=> return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username){
        Ok(_)=>Ok(username),
        Err(e) => Err(e),
    }
}