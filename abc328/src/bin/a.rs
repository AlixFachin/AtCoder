use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }
    
    let result: usize = s.iter().filter(|c| **c <= x).sum();
    
    println!("{}",result);


}
