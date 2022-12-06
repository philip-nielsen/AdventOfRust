use std::fs::File;
use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    let input = include_str!("../input.txt");
    let mut score = 1;
    let mut first = ' ';
    let mut second = ' ';
    let mut third = ' ';

    for c in input.chars() {
        let mut singals:HashSet<char> = HashSet::new();
        let temp1 = first.clone();
        let temp2 = second.clone();
        let temp3 = third.clone();
        
        match score {
            1 => first = c,
            2 => second = c,
            3 => third = c,
            _ => {
                singals.insert(temp1);
                singals.insert(temp2);
                singals.insert(temp3);
                singals.insert(c);
                if singals.len() == 4 {
                    break;
                } else {
                    first = second;
                    second = third;
                    third = c;
                }
            }
        };
        score += 1;
        
    }
    println!("{}",score);
}
