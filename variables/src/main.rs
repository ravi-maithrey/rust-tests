fn main() {
    let x = 5;
    let mut y = 6;
    println!("x is {}", x);
    //x = 6; this won't work cause x is not mutable
    y = 5;
    println!("x is {} and y is {}", x, y);
}
