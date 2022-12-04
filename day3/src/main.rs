use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut score = 0;
    let mut count = 0;
    let mut first_row = Vec::new();
    let mut second_row = Vec::new();

    for line in buffered.lines() {
        let x = line?;
        let char_vec: Vec<char> = x.chars().collect();
        
        count += 1;
        if count % 3 ==  1 {
            first_row = char_vec;
        } else if count % 3 == 2 {
            second_row = char_vec;
        } else {
            for i in 0 .. char_vec.len() - 1 {
                if first_row.contains(char_vec.get(i).unwrap_or(&'n'))  && second_row.contains(char_vec.get(i).unwrap_or(&'n')) {
                    let code = char_vec.get(i).unwrap().clone() as u32;
                    if code < 91 {
                        score += code - 38;
                    } else {
                        score += code - 96;
                    }
                 break;
                }
            }
        }
    }

    println!("{}", score);
    Ok(())
}

fn part1() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut score = 0;

    for line in buffered.lines() {
        let x = line?;
        let char_vec: Vec<char> = x.chars().collect();
        let mid = char_vec.len()/2;
        let (first, second) = char_vec.split_at(mid);
    
        for i in 0..mid {
            let code = first.get(i).unwrap().clone() as u32;
             if second.contains(first.get(i).unwrap()) {
                 if code < 91 {
                     score += code - 38;
                 } else {
                     score += code - 96;
                 }
                 break;
             }
        }
    }

    println!("{}", score);
    Ok(())
}