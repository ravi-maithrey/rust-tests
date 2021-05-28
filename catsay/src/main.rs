extern crate structopt; //using an external crate

use structopt::StructOpt; 


#[derive(StructOpt)]
struct Options{
    #[structopt(default_value = "Meow!")] /// What does the cat say?
    message: String, // defining the options for the struct which generated parameters

    #[structopt(short = "d", long = "dead")] /// Make the cat appear dead with x'd out eyes
    dead: bool,
}

fn main() {
    let options = Options::from_args(); // calling the args from the Options struct defined above
    let message = options.message; // calling the arg names message from the stuct called above
    let eye = if options.dead {"x"} else {"o"};
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("      ( {eye} {eye} )", eye=eye);
    println!("      =( I )=");
}
