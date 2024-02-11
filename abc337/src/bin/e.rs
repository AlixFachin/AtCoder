
use proconio::input;
use std::io::{stdout, Write, BufReader};
use proconio::source::line::LineSource;

// This one was a bit tricky because it is a interactive pattern

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    };

    let m = f64::log2(n as f64).ceil() as usize;

    let mut bottles: Vec<Vec<usize>> = vec![vec![];m];
    let mut solutions: Vec<String> = vec![];
    for i in 0..n {
        // Getting the binary writing of i
        let s = format!("{:b}", i);
        solutions.push(s.clone());
        for (j,c) in s.chars().rev().enumerate() {
            if c == '1' {
                bottles[j].push(i+1);
            }
        }
    }

    println!("{}", m);
    for b in bottles {
        print!("{} ", b.len());
        for i in b {
            print!("{} ", i);
        }
        println!();
    }
    stdout().flush();

    input! {
        from &mut source,
        s: String,
    }

    for (k,sol) in solutions.iter().enumerate() {
        if sol.eq(s.as_str()) {
            println!("{}", k+1);
            return;
        }
    }
    println!("1");

}
