use std::fs::File;
use std::io::{BufRead,BufReader};

struct CPU {
    cycle_count: i32,
    register: i32,
    values: Vec<i32>, 
    signal_level: i32,
    signal_strs: Vec<i32>,
    leds: Vec<char>,
    i: i32,
}

impl CPU {
    fn read_instuctions(&mut self, instructions: Vec<String>) {
        for ins in instructions {
            match ins { 
                ins if ins.contains("noop") => {
                    Self::tick(self); 
                },
                ins if ins.contains("addx") => {  
                    Self::tick(self);  
                    let mut s = ins.split(" ");
                    s.next();
                    let value = s.next().unwrap().parse::<i32>().unwrap(); 
                    Self::tick(self);
                    self.register += value; 
                    self.values.push(value);
                },
                _ => ()
            }   
        } 
        let mut i = 0;
        for c in &self.leds {
            if i==40 {
                i=0;
                println!("");
            };
            print!("{}", c);
            i+=1;
        }
    }
    fn tick(&mut self) {
        self.cycle_count += 1;

        //Part two START
        let sprite = self.register-1..self.register+2; 
        if self.leds.len() % 40 == 0 {
            self.i = 0;
        }; 
        if sprite.contains(&self.i) { self.leds.push('#') } else { self.leds.push('.') };
        self.i+=1;
        //Part two END

        let nth_signal = 20 * self.signal_level; 
        if self.cycle_count == 20 || self.cycle_count == nth_signal {
            self.signal_level += 2;
            let x = self.values.iter().sum::<i32>();
            let y = self.cycle_count * x;
            self.signal_strs.push(y);
            let final_ans = self.signal_strs.iter().sum::<i32>();
            println!("final ans: {}", final_ans);
        }
    } 
}

fn main() {
    let f = File::open("./day10.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap());
    };

    let mut cpu = CPU {
        cycle_count: 0,
        register: 1,
        values: vec![1], 
        signal_level: 1,
        signal_strs: vec![],
        leds: vec![],
        i: 0
    };

    cpu.read_instuctions(lines);

}