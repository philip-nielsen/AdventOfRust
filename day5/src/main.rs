use std::fs::File;
use std::io::{ BufReader, BufRead, Error};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;

    let (crates_columns, moves) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let (crates, colunmns) = crates_columns.split_once("\n ").unwrap();

    let test1 = crates.chars();
    let test2 = crates.split("\n");
    let test3 = test2.clone().next().unwrap();
    println!("{}", test3);

    let q = colunmns.split_whitespace();
    for p in q {
        println!("{}", p);
    }


    println!("{}", crates);
    println!("{}", colunmns);
    println!("{}", crates.get(0..20).unwrap());
    // let buffered = BufReader::new(input);

    // for line in buffered.lines() {
    //     let x = line?;
    //     let test = x.split_ascii_whitespace();
    //     println!("{}", test.clone().last().unwrap_or("default"));
    //     println!("{}", test.clone().count());
    Ok(())
        
}