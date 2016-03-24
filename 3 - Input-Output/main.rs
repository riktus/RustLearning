use std::io;

fn main(){
    println!("Enter word: ");

    let mut word = String::new();
    io::stdin().read_line(&mut word);

    println!("You entered: {}", word);
}
