use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        cookies: [Chars;h],
    }

    println!("{:?}",cookies);

    let mut shouldStop = false;
    while !shouldStop {
        // Cleaning rows
        for i in 0..cookies.len() {
            let row = cookies[i].clone();
            
        }

    }

}
