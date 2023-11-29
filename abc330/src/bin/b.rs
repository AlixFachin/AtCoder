use proconio::input;

// This problem is actually only about finding the minimum
// of a function in the interval [l,r]

fn main() {
    
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize;n],
    }

    for i in 0..n {
        if a[i] <= l {
            println!("{}",l);
        } else if a[i] >= r {
            println!("{}",r);
        } else {
            println!("{}", a[i]);
        }
    }


}
