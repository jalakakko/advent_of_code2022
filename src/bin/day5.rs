use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn main() {
    let f = File::open("./day5.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines = vec![];
    for line in reader.lines() { 
        lines.push(line.unwrap()); 
    };

    let mut cmds = vec![];
    for l in lines {
        let mut s = l.split(' '); 
        let nums = (
            s.next(),s.next(),s.next(),s.next(),s.next(),s.next()
        );
        let cmd = (
            nums.1.unwrap().parse::<usize>().unwrap(),
            nums.3.unwrap().parse::<usize>().unwrap(),
            nums.5.unwrap().parse::<usize>().unwrap()
        );
        cmds.push(cmd);
    }

    //         [Q] [B]         [H]        
    //     [F] [W] [D] [Q]     [S]        
    //     [D] [C] [N] [S] [G] [F]        
    //     [R] [D] [L] [C] [N] [Q]     [R]
    // [V] [W] [L] [M] [P] [S] [M]     [M]
    // [J] [B] [F] [P] [B] [B] [P] [F] [F]
    // [B] [V] [G] [J] [N] [D] [B] [L] [V]
    // [D] [P] [R] [W] [H] [R] [Z] [W] [S]
    //  1   2   3   4   5   6   7   8   9 
    
    let mut crates = vec![]; 
    crates.push(vec!['D','B','J','V']);
    crates.push(vec!['P','V','B','W','R','D','F']);
    crates.push(vec!['R','G','F','L','D','C','W','Q']);
    crates.push(vec!['W','J','P','M','L','N','D','B']);
    crates.push(vec!['H','N','B','P','C','S','Q']);
    crates.push(vec!['R','D','B','S','N','G']);
    crates.push(vec!['Z','B','P','M','Q','F','S','H']);
    crates.push(vec!['W','L','F']);
    crates.push(vec!['S','V','F','M','R']);

    for c in cmds { 
            let len = crates[(c.1)-1].len() - c.0; 
            let mut tmp = crates[(c.1)-1].as_slice()[crates[(c.1)-1].len() - (c.0)..].to_vec();
            crates[(c.1)-1].truncate(len);
            //Part two, comment out "tmp.reverse()"
            //tmp.reverse();
            crates[(c.2)-1].append(&mut tmp);
    }

    for c in crates {
        print!("{}", c.last().unwrap());
    }
}