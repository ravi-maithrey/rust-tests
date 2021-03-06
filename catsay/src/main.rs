extern crate colored; //using colours in terminal
extern crate exitfailure;
extern crate failure; // using the failure crate for better error handling
extern crate structopt; //using an external crate //using the exit failure wrapper for the failure crate

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String, // defining the options for the struct which generated parameters

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead with x'd out eyes
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Read cat picture from a file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    // returns nothing "()" if ok and returns an error if error
    let options = Options::from_args(); // calling the args from the Options struct defined above
    let message = options.message; // calling the arg names message from the stuct called above
    let eye = if options.dead { "x" } else { "o" };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            //.expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", cat_picture);
        }

        None => {
            if message.to_lowercase() == "woof" {
                eprintln!("It's a cat, not a dog!");
            }

            println!("{}", message.yellow().on_blue());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("     ( {eye} {eye} )", eye = eye.red().bold());
            println!("     =( I )=");
        }
    }
    Ok(()) // returning the ok declared in main
}
