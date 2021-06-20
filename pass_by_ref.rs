// let us make a function which doubles the elements in 
// an array and pass it by reference

fn double(a: &mut[i32; 10]){
    for i in 0..10{
        (*a)[i] *= 2;
    }
}

fn main(){
let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
double(&mut arr);
print!("{:?}", arr);
}