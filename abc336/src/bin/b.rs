use proconio::input;

fn main() {
    input! {
      n: usize,
    };

    let mut res = n;
    let mut count = 0;
    while res != 0 {
        if res % 2 == 0 {
            count += 1;
            res /= 2;
        } else {
            break;
        }
    }
    println!("{}", count);

}
