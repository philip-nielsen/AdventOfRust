use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut score = 0;

    for line in buffered.lines() {
        let x = line?;
        let section_ranges = x.split(",");
        let mut low = 0;
        let mut high = 1000;
        let mut last_low = 100000;
        let mut last_high = -1000;

        for section_range in section_ranges {
            let range = section_range.split("-");
            high = range.clone().last().unwrap().parse().unwrap();
            low = range.min().unwrap().parse().unwrap();

            if last_low == 100000 {
                last_high = high;
                last_low = low;
            }


        }
        
        if (high >= last_high && low <= last_low) || (last_high >= high && last_low <= low){
            score += 1;
        }
    }

    println!("{}", score);
    Ok(())
}
