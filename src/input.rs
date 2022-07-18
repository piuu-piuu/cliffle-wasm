// also text_io crate can be used
// use text_io::scan;


use simple_user_input::get_input;

fn main(){
    let input: String = get_input("Please type something...");
    println!("{}",input);
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}