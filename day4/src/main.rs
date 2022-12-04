use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut score = 0;
    
    for line in buffered.lines() {
        let x = line?;
        let char_vec: Vec<char> = x.chars().collect();
        
    }

    println!("{}", score);
    Ok(())
}
