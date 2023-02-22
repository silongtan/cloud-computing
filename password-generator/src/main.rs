use clap::Parser;
use passwordgenerator::generate_password;
// use passwordgenerator::generate_password_numbers;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value_t = 8)]
   length: usize,

    // #[arg(short, long)]
    // complexity: u8,

    #[clap(long, short, action)]
    simple: bool,
}

fn main() {
    println!("A password generator, default length is 8");
    let args = Args::parse();

    println!("{}", generate_password(args.length, args.simple));
    if args.simple {
        println!("A simple password with length {}", args.length);
    } else {
        println!("A complex password with length {}", args.length);
    }
}