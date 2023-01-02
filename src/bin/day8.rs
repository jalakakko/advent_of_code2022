use std::fs::File;
use std::io::{BufRead, BufReader};

//Part two is undone
fn main() {
    let f = File::open("./day8.txt").unwrap();
    let reader = BufReader::new(f);
    let mut grid = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line.chars();
        let mut row = vec![];
        for c in chars {
            let n = c.to_digit(10).unwrap();
            row.push(( n, false))};
        grid.push(row);
    };

    // Outer ring to true
    let len = grid.len();
        // Top row
    for row in &mut grid[..1] {
        for n in row {
            n.1 = true;
        };
    };
        // Bottom row
    for row in &mut grid[len-1..] {
        for n in row {
            n.1 = true;
        };
    };
        // Left side, then right side
    for row in &mut grid {
        let len = row.len();
        for n in &mut row[..1] {
            n.1 = true;
        };
        for n in &mut row[len-1..] {
            n.1 = true;
        };
    };

    //from left to right 
    for row in &mut grid {
        let mut highest = 0;
        for n in &mut *row { 
            if n.0 > highest {
                n.1 = true;
                highest = n.0;
            };
        };
    };

    //from right to left 
    for row in &mut grid {
        row.reverse();
        let mut highest = 0;
        for n in &mut *row {
            if n.0 > highest {
                n.1 = true;
                highest = n.0;
            }; 
        };
    };
    for row in &mut grid {
        row.reverse();
    }

    //from top to bottom
    let mut i = 0;  
    while i < grid[0].len() {
        let mut highest = 0;
        for row in &mut grid {
            for n in &mut row[i..i+1] {
                if n.0 > highest {
                    n.1 = true;
                    highest = n.0;
                };
            };
        };
        i+=1;
    };
    
    // from bot to top
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let column_len = grid.len();
    let row_len = grid[0].len();
    let mut highest = 0; 
    while i < row_len * column_len { 
        if k == column_len {  
            j+=1;
            k = 0;
            highest = 0;  
        }; 
        
        for row in &mut grid[(column_len-1)-k..(column_len)-k] {
            for n in &mut row[j..j+1] { 
                if n.0 > highest {
                    n.1 = true;
                    highest = n.0;
                };
            };
        };
        i+=1;
        k+=1;
    };

    let mut visibles = 0;
    for row in &mut grid {
        for n in row {
            if n.1 == true { visibles+=1; } 
        };
    };
    println!("{}", visibles);
}