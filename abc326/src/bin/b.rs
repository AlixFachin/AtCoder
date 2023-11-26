use proconio::input;

fn is_326_number(x: usize) -> bool {
    let a = x % 10;
    let b = (x /10) % 10;
    let c = (x/100) % 10;

    return b*c == a;

}



fn main() {

    input! {
        n: usize,
    }

    for x in n..1000 {
        if is_326_number(x) {
            println!("{}",x);
            break;
        }
    }

}
