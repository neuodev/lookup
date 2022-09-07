use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use thiserror::Error;

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
    /// Show lines by index
    #[clap(short, long)]
    lines: bool,
}

#[derive(Debug, Error)]
enum AppErr {
    #[error("File not found")]
    FileNotFound(String),
    #[error("Error while reading the file")]
    IoErr(#[from] std::io::Error),
    #[error("Invalid regex")]
    RegexErr(#[from] regex::Error),
}

fn main() -> Result<(), AppErr> {
    let args = Args::parse();
    let path = Path::new(&args.file);

    if !path.exists() {
        return Err(AppErr::FileNotFound(args.file));
    }

    let re = Regex::new(&args.regex)?;

    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            let mut new_line = line.clone();
            re.captures_iter(&line).for_each(|caps| {
                let keyword = &caps[0];
                println!("{}", &format!("{}", keyword).underline());
                new_line = line.replace(keyword, &format!("{}", keyword).underline());
            });
            // let line = format!("{}", line).underline().red().bold();
            if args.lines {
                println!("[{idx}] {}", new_line);
            } else {
                println!("{}", new_line);
            }
        }
    }

    Ok(())
}
