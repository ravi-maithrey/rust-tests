fn main(){
    enum Cont {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let continent = Cont::Africa;
    match continent{
        Cont::Africa => print!("Africa"),
        Cont::America => print!("America"),
        Cont::Asia => print!("Asia"),
        Cont::Europe => print!("Europe"),
        Cont::Oceania => print!("Oceania"),
    }
}