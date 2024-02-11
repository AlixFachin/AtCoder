use proconio::input;

fn main() {
    input! {
       q : usize,
       commands: [(char, usize); q],
    };

    let mut a: Vec<usize> = vec![];
    for i in 0..q {
        let (c,x) = commands[i];
        if c == '1' {
            a.push(x);
        } else {
            let n = a.len();
            println!("{}", a[n-x]);
        }
    }

}
