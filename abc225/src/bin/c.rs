use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       m: usize,
       b: [[usize;m];n],
    };

    let root = b[0][0];
    let root_i = (root-1) / 7;
    let root_j = (root-1) % 7;
    for i in 0..n {
        for j in 0..m {
            let x= b[i][j];
            if DEBUG {
                println!("root_i: {root_i}, root_j: {root_j} i:{i} j:{j} x:{x}");
            }
            if (x-1) / 7 != root_i + i || (x-1) % 7 != root_j + j {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");

}
