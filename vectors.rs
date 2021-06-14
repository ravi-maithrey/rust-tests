fn main(){
    let mut x = vec!["hello"];
    println!("{}", x.len());
    x.push("world");
    x.insert(1, "line");
    x.insert(2, "contains");
    x.remove(3);
    x.push("about Rust");
    x.pop();
    println!("{}", x.len());
    for i in 0..x.len(){
    println!("{}", x[i]);
    }
    println!("{:?}", x);
}