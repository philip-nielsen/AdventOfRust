use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";

    //let mut output = File::create(path)?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut biggestSum: i32 = 0;
    let mut secondSum: i32 = 0;
    let mut thirdSum: i32 = 0;
    let mut tempSum: i32 = 0;

    for line in buffered.lines() {
        //println!("{}", line?);
        let x = line?;
        if x != "" {
            let num: i32 = x.parse().unwrap();

            tempSum += num;
            //println!("{}", num);
        } else {
            if tempSum > biggestSum {
                thirdSum = secondSum;
                secondSum = biggestSum;
                biggestSum = tempSum;
                
            } else if tempSum > secondSum {
                thirdSum = secondSum;
                secondSum = tempSum;
            } else if tempSum > thirdSum {
                thirdSum = tempSum;
            }
            tempSum = 0;
        }
    }

    println!("{}", biggestSum);
    println!("{}", secondSum);
    println!("{}", thirdSum);


    let sum: i32 = biggestSum + secondSum + thirdSum;
    println!("{}", sum);
    Ok(())
}