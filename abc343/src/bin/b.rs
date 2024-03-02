use proconio::input;

fn main() {
    input! {
       n: usize,
       a: [[usize; n]; n],
    };

    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                print!("{} ", j+1);
            }
        }
        println!("");
    }


}
