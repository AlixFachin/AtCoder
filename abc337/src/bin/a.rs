use proconio::input;

fn main() {
    input! {
       n: usize,
       scores: [(i32,i32); n]
    };

    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        let (a, b) = scores[i];
        x += a;
        y += b;
    }

    if x > y {
        println!("Takahashi");
    } else if x < y {
        println!("Aoki");
    } else {
        println!("Draw");
    }

}
