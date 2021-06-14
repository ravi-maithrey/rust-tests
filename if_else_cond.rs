fn main(){
    let n = 4;

    println!("{}",
        if n > 1000{
            "big"
        }
        else if n > 0{
            "small"
        }
        else if n < 0{
            "negative"
        }
        else{
            "neither postive nor negative"
        });
    
    let mut i = 0;
    while i < 10{
        print!("{} ", i*i);
        i += 1;
    }
    }

