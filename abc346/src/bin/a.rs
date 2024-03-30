use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       a: [usize; n],
    };

    for i in 0..(n-1) {
        println!("{}", a[i]*a[i+1]);
    }

}
