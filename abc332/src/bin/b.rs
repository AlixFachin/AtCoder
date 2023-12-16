use proconio::input;

fn main() {
    input! {
       k: usize,
       g: usize,
       m: usize,
    };

    let mut glass: usize = 0;
    let mut mug: usize = 0;

    for _i in 0..k {

        if glass == g {
            glass = 0;
        } else if mug == 0 {
            mug = m;
        } else {
            let until_full = g - glass;
            let qty_transferred = until_full.min(mug);
            mug = mug - qty_transferred;
            glass = glass + qty_transferred;
        }
    }

    println!("{} {}", glass, mug);

}
