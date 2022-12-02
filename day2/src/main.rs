use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    
    let mut score: i32 = 0;

    for line in buffered.lines() {
        let x = line?;
        let char_vec: Vec<char> = x.chars().collect();
        let first = char_vec[0];
        let last = char_vec[char_vec.len() - 1];

            match first {
                'A' => match last {
                    'X' => score += 3,
                    'Y' => score += 4,
                    _ => score += 8,
                },
                'B' => match last {
                    'X' => score += 1,
                    'Y' => score += 5,
                    _ => score += 9,
                },
                _ => match last {
                    'X' => score += 2,
                    'Y' => score += 6,
                    _ => score += 7,
                },
            };
    }

    println!("{}", score);

    Ok(())
}