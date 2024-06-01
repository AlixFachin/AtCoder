use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       m: usize,
       a: [usize; n],
       b: [usize; m],
    };

    let mut c: Vec<(usize, char)> = vec![];
    for i in a {
        c.push((i, 'a'));
    }
    for i in b {
        c.push((i, 'b'));
    }
    
    c.sort_by(|a,b| a.0.cmp(&b.0));

    if DEBUG {
        println!("{:?}", c);
    }

    for i in 0..(c.len()-1) {
        if c[i].1 == c[i+1].1 && c[i].1 == 'a' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
