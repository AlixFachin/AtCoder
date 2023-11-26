use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize;n],
    }

    let nr_score = a.iter().filter(|x| **x >= l).count();
    println!("{}", nr_score);

}
