use proconio::input;

fn main() {
    
    input! {
        n: usize,
        m: usize,
        a: [usize;n],
        b: [usize;m],
    }

    let mut score: usize = 0;

    for test_index in b {
        score = score + a[test_index-1];
    }

    println!("{}", score);

}
