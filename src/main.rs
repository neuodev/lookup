use clap::Parser;

/// Search files line by line using regular expression
#[derive(Parser, Debug)]
#[clap(author = "Ahmed Ibrahim", version, about, long_about = None)]
struct Args {
    /// File to search
    #[clap(short, long, value_parser)]
    file: String,
    /// Regular experssion
    #[clap(short, long, value_parser)]
    regex: String,
}

fn main() {
    let args = Args::parse();
}
