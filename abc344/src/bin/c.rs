use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       n: usize,
       a: [usize;n],
       m: usize,
       b: [usize;m],
       l: usize,
       c: [usize;l],
       q: usize,
       x: [usize;q],
    };

    let mut multiples_set: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                multiples_set.insert(a[i] + b[j] + c[k]);
            }
        }
    }

    if DEBUG {
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", multiples_set);
    }

    for i in 0..q {
        if multiples_set.contains(&x[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }





}
