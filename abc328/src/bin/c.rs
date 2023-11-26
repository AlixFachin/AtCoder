use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [[usize;2];q],
    }
    
    let mut doubles_repartition = vec![];
    let mut nr_doubles: usize = 0;
    doubles_repartition.push(0);
    for i in 0..n-1 {
        if s[i] == s[i+1] {
            nr_doubles = nr_doubles + 1;
        }
        doubles_repartition.push(nr_doubles);
    }
    doubles_repartition.push(nr_doubles);

    if DEBUG { println!("{:?}", doubles_repartition)}

    // Now we just need for each interval to count how many doubles are there.
    for i in 0..q {
        let l = queries[i][0];
        let r = queries[i][1];

        if l == r {
            println!("0");
        } else {
            let result = doubles_repartition[r-1] - doubles_repartition[l-1];
            println!("{}", result);
        }

    }

}
