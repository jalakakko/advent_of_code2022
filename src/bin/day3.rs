use std::{
    fs::File,
    io::{BufReader, BufRead}
};

#[derive(Debug)]
struct Rucksack {
    content: String,
    id: char,
}

impl Rucksack {  
    fn new(content: String) -> Self {
        let id = Self::identify(content.to_string());
        Rucksack {
            content: content.to_string(),
            id: id, 
        }
    }
    
    fn identify(content: String) -> char { 
        let split = content.split_at(content.len() / 2);
        let mut id = ' ';
        for x in split.0.chars() {
            for y in split.1.chars() {
                if x==y { id = x; break; }
            }
        }
        id
    } 
}

fn main() { 
    let file = File::open("./day3.txt").unwrap();
    let reader = BufReader::new(file); 

    let mut x: Vec<u32> = (97..123).collect();
    x.append(&mut (65..91).collect());
    let y = x.into_iter()
        .map(|v| char::from_u32(v).unwrap())
        .collect::<Vec<char>>();
    let mut char_values = vec![];
    for mut i in y.iter().enumerate() {
        i.0 += 1;
        char_values.push(i);
    };

    //Part one
    let mut sum = vec![];
    for line in reader.lines() {
        let r = Rucksack::new(line.unwrap()); 
        for val in &char_values {
            if *val.1 == r.id {  
                sum.push(val.0 as u32);
                break;
            }
        }
    }
    let sum: u32 = sum.iter().sum();
    println!("{}", sum);

    //Part two
    let file = File::open("./day3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    let mut groups = vec![];
    for chunk in lines.chunks(3) {
        groups.push(chunk);
    }
    
    let mut types: Vec<char> = vec![];
    for group in groups {
        let mut found = false;
        for c0 in group[0].chars() {
            if found { break; }
            for c1 in group[1].chars() {
                if found { break; }
                for c2 in group[2].chars() {
                    if c0 == c1 && c0 == c2 {
                        types.push(c0);
                        found = true;
                        break;
                    }
                }
            }
        }
    }

    let mut sum = vec![];
    for t in &types {
        for val in &char_values {
            if *val.1 == *t {  
                sum.push(val.0 as u32);
                break;
            }
        }
    }
    let sum: u32 = sum.iter().sum();
    println!("{}", sum);
}