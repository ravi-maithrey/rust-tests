fn main(){
    struct SomeData{
        integer: i32,
        fraction: f32,
        character: char,
        five_bytes: [u8; 5],
    }

    let data = SomeData{
        integer: 500000,
        fraction: 134.98,
        character: 'R',
        five_bytes: [10, 11, 43, 50, 2],
    };

    print!("{}, {}, {}, {}",
        data.five_bytes[3], data.integer,
        data.fraction, data.character);
}