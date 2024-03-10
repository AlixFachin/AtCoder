use proconio::input;

const DEBUG:bool = false;

fn main() {

    let mut result: Vec<usize> = vec![];
    let mut last_value: usize;

    input! {
        x: usize,
    }
    last_value = x;
    while last_value > 0 {
        result.push(last_value);
        input! {
            x: usize,
        }
        last_value = x;
    }

    println!("0");
    for x in result.into_iter().rev() {
        println!("{}", x);
    }

}
