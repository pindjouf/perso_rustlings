//import args parser, patharg confirm thing, filesystem manip operations module
use clap::Parser;
use patharg::InputArg;
use std::fs;

//define args structure
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    //#[arg(short, long, required = true)]
    //flag: String,

    #[arg(required = true)]
    file: InputArg,
}

fn main() {
    //fetch args into var
    let args = Args::parse();
    
    //convert arg into string
    let file_path = args.file.to_string();
   
    //match args.flag {
    //
    //}

    //return, read & print file content if ok else return and print err
    match fs::read_to_string(file_path) {
        Ok(content) => println!("Content:\n{}\n", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}

//fn counter(arg: Type) -> RetType {
//    todo!();
//}
