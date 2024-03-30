use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       x: i64,
    };
    if x == 0 {
        println!("0");
        return;
    }

    if x.abs() % 10 == 0 {
        let y = x / 10;
        println!("{y}");
        return;
    }

    if x < 0 {
        let y = -(x.abs() / 10);
        println!("{y}");
        return;
    }
    let y = (x / 10) + 1;
    println!("{y}");
}
