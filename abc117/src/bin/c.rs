use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       m: usize,
       mut x: [i32; m],
    };

    if n >= m {
        println!("0");
        return;
    }

    x.sort();
    let distances = x.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    let mut distances = distances.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    distances.sort_by(|a, b| a.1.cmp(b.1));

    if DEBUG {
        println!("{:?}", distances);
    }

    let mut ans = 0;
    for i in 0..m-n {
        ans += distances[i].1;
    }

    println!("{}", ans);

}
