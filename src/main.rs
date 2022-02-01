use clap::Parser;

mod lex;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_location: String,
}

fn main() {
    let args = Args::parse();
    // LEX
    lex::parse(args.file_location)
}
