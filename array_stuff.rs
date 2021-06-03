fn main(){
    let mut arr = [1, 2, 3, 4];
    let arr1 = ["hello", "world"];
    let arr2 = [true, false];
    println!("{}", arr[2]);
    arr[2] = 21;
    println!("{}", arr[2]);
    print!("Lengths are {}, {}, {}", arr.len(), arr1.len(), arr2.len());
}