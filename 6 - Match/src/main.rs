fn main() {
    let number = 4;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something bigger"),
    };

    println!("");

    match number{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),
        _ => println!("something else"),
    }
}
