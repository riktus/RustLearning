struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

fn main() {
    //something like ctor with arguments
    let circle = Circle::new(0.0, 0.0, 2.0);

    println!("{}", circle.radius);
}
