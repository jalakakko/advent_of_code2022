use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Elf {
    name: String,
    calories: Vec<u32>,
}
impl Elf {
    fn new(name: String, calories: Vec<u32>) -> Self {
        Elf {
            name: name,
            calories: calories,
        }
    }

    fn total(&self) -> u32 {
        let mut total = 0;
        for c in &self.calories {
            total += c;
        }
        total
    }
}

fn main() {
    let file = fs::File::open("./day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut index = 1; 
    let mut elves: Vec<Elf> = vec![];
    let mut calories: Vec<u32> = vec![];

    for line in reader.lines() { 
        if line.as_ref().unwrap().is_empty() { 
            let elf = Elf::new(format!("Elf {index}"), calories.clone());
            index += 1; 
            elves.push(elf);
            calories.clear();
        } else {
            calories.push(line.as_ref().unwrap().parse::<u32>().unwrap());
        };
    }
 
    //Part one
    let mut highest = 0;
    for e in &elves { 
        if highest < e.total() {
            highest = e.total();
        }; 
    };
    println!("{}", highest);

    //Part two
    let mut highest_three = 0;
    elves.sort_by(|a,b| a.total().cmp(&b.total()));
    let three = elves.as_slice()[elves.len()-3..].to_vec();
    for e in three {
        highest_three += e.total();
    }
    println!("{}", highest_three);
    
}
