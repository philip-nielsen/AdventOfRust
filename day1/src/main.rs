use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    
    let mut biggest_sum: i32 = 0;
    let mut second_sum: i32 = 0;
    let mut third_sum: i32 = 0;
    let mut temp_sum: i32 = 0;

    for line in buffered.lines() {
        let x = line?;
        if x != "" {
            let num: i32 = x.parse().unwrap();
            temp_sum += num;
        } else {
            if temp_sum > biggest_sum {
                third_sum = second_sum;
                second_sum = biggest_sum;
                biggest_sum = temp_sum;
                
            } else if temp_sum > second_sum {
                third_sum = second_sum;
                second_sum = temp_sum;
            } else if temp_sum > third_sum {
                third_sum = temp_sum;
            }
            temp_sum = 0;
        }
    }
    let sum: i32 = biggest_sum + second_sum + third_sum;
    print!("{}", "answer part 1:");
    println!("{}", biggest_sum);
    print!("{}", "answer part 2:");
    println!("{}", sum);
    Ok(())
}