use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       t: usize,
       a: [usize; t],
    }

    // We will keep the count of the numbers of elements in each row, each column and for the two diagonals
    // Therefore we have a bingo if the count of any of these is equal to n

    let mut rows_count: Vec<usize> = vec![0; n];
    let mut cols_count: Vec<usize> = vec![0; n];
    let mut diag1_count: usize = 0;
    let mut diag2_count: usize = 0;

    for (i, x) in a.iter().enumerate() {
        // Checking at which row does (x-1) belong
        let row = (*x-1) / n;
        rows_count[row] += 1;

        let col = (*x-1) % n;
        cols_count[col] += 1;

        if row == col {
            diag1_count += 1;
        }
        if row + col == n-1 {
            diag2_count += 1;
        }

        if (rows_count[row] == n) || (cols_count[col] == n) || (diag1_count == n) || (diag2_count == n) {
            println!("{}", i+1);
            return;
        }
    }
    println!("-1");
}
