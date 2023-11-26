use proconio::input;

fn main() {

    input! {
        n: usize,
        mut a: [usize;n],
    }

    a.sort();
    a.dedup();

    let m = a.len();
    println!("{}", a[m-2]);

}
