use proconio::input;
fn main() {
    input! {
        n: usize,
        base: [(usize, usize);n],
    }

    println!("{:?}",base);

    let mut start_hour:Vec<usize> = vec![];
    let mut end_hour:Vec<usize> = vec![];
    for i in 0..n {
        start_hour.push((9 + 24 - base[i].1) %24 );
        end_hour.push((18 + 24 - base[i].1) % 24);
    }




}
