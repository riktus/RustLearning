trait Printer {
    fn out(&self);
}

impl Printer for u8 {
    fn out(&self) { println!("u8: {}", *self); }
}

impl Printer for String {
    fn out(&self) { println!("string: {}", *self); }
}

impl Printer for i32{
    fn out(&self) { println!("i32: {}", *self); }
}

fn work<T: Printer>(x: T) {
    x.out();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();
    let z = 45;

    work(x);
    work(y);
    work(z);
}
