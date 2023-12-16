use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       n: usize,
       q: usize,
       p: [Chars;n],
       queries: [(usize,usize,usize,usize);q],
    };

    let mut grid: Vec<Vec<char>> = vec![vec!['W';n];n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = p[i][j];
        }
    }

    for (a,b,c,d) in queries {

        let mut num_black_cells = 0;
        for i in a..=c  {
            for j in b..=d {
                if grid[i%n][j%n] == 'B' {
                    num_black_cells = num_black_cells + 1;
                }
            }
        }
        println!("{}", num_black_cells);
    }


}
