use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{cmp::*, vec};

fn main() {
    let file = File::open("./day4.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(',');
        let pairs = ( split.next().unwrap().to_string(),
            split.next().unwrap().to_string() );
        lines.push(pairs);
    }; 

    let mut part_one_ans = 0; 
    let mut part_two_ans = 0;

    for line in lines { 
        // E.g. input: 3-94,3-96
        let mut split1 = line.0.split('-');
        let p1 = ( split1.next().unwrap().parse::<usize>().unwrap(),
            split1.next().unwrap().parse::<usize>().unwrap() );
        // p1 = 3-94

        let mut split2 = line.1.split('-');
        let p2 = ( split2.next().unwrap().parse::<usize>().unwrap(),
        split2.next().unwrap().parse::<usize>().unwrap() );
        // p2 = 3-96
        
        let max = max(p1.1, p2.1);
        let mut vec1 = vec![0; max];
        let mut vec2 = vec![0; max]; 
        // two max sized(96) vectors, filled with 0's

        vec1.splice(
            (p1.0)..(p1.1),
            vec![1; (p1.1 - p1.0) + 1]
        ); 
        vec2.splice(
            (p2.0)..(p2.1), 
            vec![1; (p2.1 - p2.0) + 1]
        );

        if vec1[p1.0] == vec2[p1.0] && vec1[p1.1] == vec2[p1.1] 
        || vec1[p2.0] == vec2[p2.0] && vec1[p2.1] == vec2[p2.1] {
            part_one_ans += 1;
        }

    //        v1[p1.0]   v1[p1.1] 
    //           V           V
    // v1 .......|||||||||||||..............
    // v2 ....||||||||||||||||||||||||......
    //           ^           ^ 
    //        v2[p1.0]    v2[p1.1] 


    //     v1[p2.0]              v1[p2.1] 
    //        V                      V
    // v1 .......|||||||||||||..............
    // v2 ....||||||||||||||||||||||||......
    //        ^                      ^ 
    //     v2[p2.0]              v2[p2.1]

    // Part two
        if vec1[p1.0] == vec2[p1.0] || vec1[p1.1] == vec2[p1.1]
        || vec1[p2.0] == vec2[p2.0] || vec1[p2.1] == vec2[p2.1] {
            part_two_ans += 1;
        }
    } 
    println!("{}", part_one_ans);
    println!("{}", part_two_ans);

}
 