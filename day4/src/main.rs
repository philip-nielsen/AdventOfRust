use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut score = 0;
    let mut low = 0;
    let mut high = 1000;
    let mut last_low = 100000;
    let mut last_high = -1000;

    for line in buffered.lines() {
        let x = line?;
        let section_ranges = x.split(",");
        let mut count = 0;
        for section_range in section_ranges {
            
            let range = section_range.split("-");

            if count == 1 {
                last_high = high;
                last_low = low;
            }
            count += 1;
            high = range.clone().last().unwrap().parse().unwrap();
            low = range.clone().next().unwrap().parse().unwrap();

            if last_low == 100000 {
                last_high = high;
                last_low = low;
            }
        }

        if high >= last_low && low <= last_high {
            score += 1;
        }
    }

    println!("{}", score);
    Ok(())
}

// fn part1() -> Result<(), Error> {
//     let path = "input.txt";
//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);
//     let mut score = 0;
//     let mut low = 0;
//     let mut high = 1000;
//     let mut last_low = 100000;
//     let mut last_high = -1000;

//     for line in buffered.lines() {
//         let x = line?;
//         let section_ranges = x.split(",");
//         let mut count = 0;
//         for section_range in section_ranges {
            
//             let range = section_range.split("-");

//             if count == 1 {
//                 last_high = high;
//                 last_low = low;
//             }
//             count += 1;
//             high = range.clone().last().unwrap().parse().unwrap();
//             low = range.clone().next().unwrap().parse().unwrap();

//             if last_low == 100000 {
//                 last_high = high;
//                 last_low = low;
//             }
//         }

        
//         println!("{} - {}", last_low, last_high);
//         println!("{} - {}", low, high);
//         if (high >= last_high && low <= last_low) || (last_high >= high && last_low <= low){
//             score += 1;
//         }
//     }
//     Ok(())
// }