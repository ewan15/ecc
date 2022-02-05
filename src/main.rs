use clap::Parser;

mod lex;
mod parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_location: String,
}

fn main() {
    let args = Args::parse();
    // LEX
    let tokens = lex::parse(args.file_location);
    let ast = parser::construct_ast(tokens);
}
