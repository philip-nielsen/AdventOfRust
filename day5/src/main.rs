use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let x = line?;
        let test = x.split_ascii_whitespace();
        println!("{}", test.clone().last().unwrap_or("default"));
        println!("{}", test.clone().count());
        
    }
    Ok(())
}