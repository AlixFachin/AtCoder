use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       a: i32,
       b: i32,
       c: i32,
       d: i32,
    };

    println!("{}", (a+b)*(c-d));
    println!("Takahashi");

}
