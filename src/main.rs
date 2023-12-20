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

fn main() {
    let cli = Cli::parse();

    if cli.newline {
        print!("{:?}", cli.text);
    } else {
        println!("{:?}", cli.text)
    }
    
}