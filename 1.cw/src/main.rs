use clap::Parser;
use patharg::InputArg;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    #[arg(short, long)]
    chars: bool,

    #[arg(required = true)]
    file: InputArg,
}

fn main() {
    let args = Args::parse();
    
    let file_path = args.file.to_string();
   
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            match (args.lines, args.words, args.chars) {
                (true, false, false) => println!("There are {} lines in {}", count_lines(&content), file_path),
                (false, true, false) => println!("There are {} words in {}", count_words(&content), file_path),
                (false, false, true) => println!("There are {} characters in {}", count_chars(&content), file_path),
                _ => println!("Nothing."),
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_chars(content: &str) -> usize {
    let mut char_count = 0;
    for _i in content.chars() {
        char_count += 1;
    }
    "usize" ;
    char_count
}
