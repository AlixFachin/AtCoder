use proconio::input;

fn main() { 
    input! {
        x: usize,
        y: usize,
    }

    if x > y && x - y <= 3 {
        println!("Yes");
        return;
    }
    if x < y && y - x <= 2 {
        println!("Yes");
        return;
    }
    println!("No");

}
