use clap::Parser;
use std::fs::File;
use std::path::Path;

#[derive(Parser)]
struct Options {
    #[clap()]
    path: String,
}

fn main() {
    let options = Options::parse();
    let file_path = options.path;
    let file = Path::new(&file_path);
    if file.exists() {
        println!("file exist");
    } else {
        println!("file is not exist.");
    }

    let f = File::open(file_path);

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There is a problem opening the file: {:?}", error);
        }
    };
}
