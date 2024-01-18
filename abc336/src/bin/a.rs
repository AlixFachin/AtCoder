use proconio::input;

fn main() {
    input! {
       n: usize
    };

    let mut res: Vec<char> = vec!['L'];
    for i in 0..n {
        res.push('o');
    }
    res.push('n');
    res.push('g');
    println!("{}", res.into_iter().collect::<String>());



}
