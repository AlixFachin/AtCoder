/**
 * ABC 240 - "Jumpin Takahashi"
 * https://atcoder.jp/contests/abc240/tasks/abc240_c
 */
use proconio::input;
const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       x: usize,
       jumps: [(usize,usize);n],
    }

    // DP array where we store DP[i][j] = "is it possible to reach the value j with i jumps?" 
    // note that for cleaner indexing we will need an array with x+1 length. (0-indexing and all that)
    let mut DP: Vec<Vec<bool>> = vec![vec![false;x+1];n];

    let (a0,b0) = jumps[0];
    if a0 <= x { DP[0][a0] = true;}
    if b0 <= x { DP[0][b0] = true; }
    for i in 1..n {
        let (a,b) = jumps[i];
        for j in 0..x {
            if DP[i-1][j] {
                // We can go to j in i-1 steps
                if j + a <= x {
                    DP[i][j+a]= true;
                }
                if j + b <= x {
                    DP[i][j+b] = true;
                }
            }
        }
    }

    if DP[n-1][x] {
        println!("Yes")
    } else {
        println!("No")
    }

}
