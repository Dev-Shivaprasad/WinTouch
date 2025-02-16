use clap::Parser;
use std::fs;

fn main() {}

fn createfile(filename: &str) {
    match fs::File::create(filename) {
        Ok(_msg) => println!("File '{}' created Successfully :)\n", filename),
        Err(er) => println!("Could not create file '{}' :( - \t - {}", filename, er),
    };
}

fn readfile(filename: &str) {
    let content = fs::read_to_string(filename);
    match content {
        Ok(ms) => println!("{}", ms),
        Err(er) => println!("Could not Load file to read :( -\t- {}", er),
    }
}

fn createdirectory(dirname: &str) {
    match fs::create_dir(dirname) {
        Ok(_msg) => println!("Directory / Folder '{}' created Successfully :)\n", dirname),
        Err(er) => println!(
            "Could not create Directory / Folder '{}' :( - \t - {}",
            dirname, er
        ),
    }
}
