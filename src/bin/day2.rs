use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("./day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points = 0;
    for line in reader.lines() {
        let mut x = line.unwrap();
        x.retain(|c| !c.is_whitespace());

        //Part two
        let y = x.split_at(1);
        match y.0 {
            "A" => match y.1 {
                    "X" => x = "AZ".to_string(),
                    "Y" => x = "AX".to_string(),
                    "Z" => x = "AY".to_string(),
                    _ => () 
                    },
            "B" => match y.1 {
                    "X" => x = "BX".to_string(),
                    "Y" => x = "BY".to_string(),
                    "Z" => x = "BZ".to_string(),
                    _ => ()
                },
            "C" => match y.1 {
                "X" => x = "CY".to_string(),
                "Y" => x = "CZ".to_string(),
                "Z" => x = "CX".to_string(),
                _ => ()
            }, 
            _ => ()
        }

        //Part one
        match x.as_str() {
            "AX" => points += 4,
            "AY" => points += 8,
            "AZ" => points += 3,
            "BX" => points += 1,
            "BY" => points += 5,
            "BZ" => points += 9,
            "CX" => points += 7,
            "CY" => points += 2,
            "CZ" => points += 6, 
            _ => ()
        } 
    }
    println!("{}", points);
}