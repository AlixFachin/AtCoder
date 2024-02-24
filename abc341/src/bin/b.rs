use proconio::input;

fn main() {
    input! {
       n: usize,
       a: [usize; n],
       change: [(usize, usize); (n-1)],
    };

    let mut currencies : Vec<usize> = vec![0; n];
    for i in 0..n {
        currencies[i] = a[i];
    }
    for i in 0..(n-1) {
        let (from, to) = change[i];
        // We'll check how many times we can convert
        let times = currencies[i] / from;
        currencies[i+1] += times * to;
    }

    println!("{}", currencies[n-1]);

}
