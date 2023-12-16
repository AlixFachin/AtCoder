use proconio::input;

fn main() {
    input! {
       n: usize,
       s: usize,
       k: usize,
       command: [(usize,usize);n],
    };

    let mut command_price = 0;
    for i in 0..command.len() {
        command_price = command_price + command[i].0 * command[i].1;
    }
    if command_price < s {
        command_price = command_price + k;
    }

    println!("{}", command_price);

}
