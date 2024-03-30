use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       k: usize,
    };

    if k < 10 {
        println!("21:0{k}");
        return;
    }
    if k < 60 {
        println!("21:{:02}", k);
        return;
    }
    if k < 70 {
        println!("22:0{}", k-60);
        return;
    }
    println!("22:{:02}", k-60);

}
