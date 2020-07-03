extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat sayy?
    message: String,
    #[structopt(short = "d",  long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead {"x"} else {"o"};
    println!("{}", message.bright_yellow().underline()
    .on_purple());
    match &options.catfile {
        Some (path) => {
            let cat_template = std::fs::read_to_string(path)
            .with_context(|_| format!("could not read file {:?}",
                path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
            Ok(())
        },
        None => {
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye=eye.red().bold());
            println!("    =( I )=");
            Ok(())
        }
    }
    
}
