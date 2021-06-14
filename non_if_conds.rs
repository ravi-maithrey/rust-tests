fn main(){
    let a = 1;
    let b = 2;
    print!("The greater number is {}", (a * ((a>b) as i32) + (b * ((a<b) as i32))));
}