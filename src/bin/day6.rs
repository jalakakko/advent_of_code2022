use std::fs;
use std::io::{self, BufRead, BufReader};

fn main() {
    let txt = fs::read_to_string("./day6.txt").unwrap();
    let chars = txt.chars().enumerate();
    let mut four_chars = vec![];
    let mut ans = 0;

    for (i, c) in chars {
        //Part 2, switch 4 to 14
        if &four_chars.len() < &4 { 
            four_chars.push(c);
            continue;
        } else { 

            let mut copy = four_chars.to_vec();
            copy.sort();
            copy.dedup();
            if copy.len() == four_chars.len() {
                ans = i;
                break;
            }

            four_chars.push(c);  
            four_chars.remove(0);
        }
    } 
    println!("{}", ans)
}


