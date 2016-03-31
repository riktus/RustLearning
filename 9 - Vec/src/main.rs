fn main() {
    let mut stack = vec![1, 2];
    println!("{:?}", stack);

    stack.pop();
    stack.push(5);
    stack.push(7);

    println!("{:?}", stack);
}
