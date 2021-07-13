// making a generic function
fn f<T>(ch: char, num1: T, num2: T) -> T{
    if ch == 'a' {num1}
    else {num2}
}

fn main(){
    let a: i16 = f('a', 32, 41);
    let b: f32 = f('b', 32.3, 43.5);
    print!("{} {}", a, b);
}