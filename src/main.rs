use std::fs::File;
use std::path::Path;
use std::io::{ self, BufRead, BufReader };
use std::env;

fn read_lines(filename: &String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.

    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open {}: {}", filename, why),
        Ok(file) => return io::BufReader::new(file).lines(),
    };
}

fn main() {
    // Stores the iterator of lines of the file in lines variable.
    let args: Vec<String> = env::args().collect();
    if args.len()>0 {
        let filename = &args[1];
        let lines = read_lines(filename);
        // Iterate over the lines of the file, and in this case print them.
        for line in lines {
            println!("{}", line.unwrap());
        }
    }
    else {
        println!("Missing Arguments");  
    }
}