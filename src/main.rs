use core::fmt;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Not append the args on a newline
    #[arg(short,long)]
    newline: bool,

    /// Argument for the standard output
    #[arg()]
    text: Vec<String>,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", self.text.iter().map(|el| format!("{}", el)).fold(String::new(), |acc, arg| acc + " " + arg.as_str()))?;

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.newline {
        print!("{}", cli);
    } else {
        println!("{}", cli)
    }
    
}