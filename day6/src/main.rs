use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let mut score = 14;

    for i in 0..input.len()-1 {
        let mut signals = HashSet::new();
        let temp = input.get(i..14+i).unwrap().chars();
        
        for c in temp {
            signals.insert(c);
        }
        if signals.len() == 14 {
            break;
        }
        score += 1;
    }
    println!("{}",score);
}

fn part1() { //Bad, very very bad solution
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
