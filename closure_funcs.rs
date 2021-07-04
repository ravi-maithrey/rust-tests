use std::cmp::Ordering;

fn desc(a: &i32, b: &i32) -> Ordering{
    if a > b {Ordering::Less}
    else if a < b {Ordering::Greater}
    else {Ordering::Equal}
}

fn main(){
    let mut arr = [1, 4, 2, 17, 6, 13];
    arr.sort_by(desc);
    print!("{:?}", arr);
}