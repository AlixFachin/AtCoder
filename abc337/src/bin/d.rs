use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       h: usize,
       w: usize,
       k: usize,
       s: [Chars; h],
    };

    // Looking at rows first
    let mut min_cost = 1000000000;
    let mut found = false;
    if k <= w {
        for i in 0..h{
            let mut numX = 0;
            let mut numO = 0;
            let mut numB = 0;

            for j in 0..k {
                if s[i][j] == 'x' {
                    numX += 1;
                } else if s[i][j] == 'o' {
                    numO += 1;
                } else {
                    numB += 1;
                }
            }
            if numX == 0 {
                let current_cost = numB;
                min_cost = min_cost.min(current_cost);
                found = true;
            }
            for j in 0..(w-k) {
                // We will add the char at j+k and remove the one at j
                if s[i][j] == 'x' {
                    numX -= 1;
                } else if s[i][j] == 'o' {
                    numO -= 1;
                } else {
                    numB -= 1;
                }
                if s[i][j+k] == 'x' {
                    numX += 1;
                } else if s[i][j+k] == 'o' {
                    numO += 1;
                } else {
                    numB += 1;
                }
                if numX == 0 {
                    let current_cost = numB;
                    min_cost = min_cost.min(current_cost);
                    found = true;
                }
            }
        }

    }

    if k <= h {
        for j in 0..w {
            let mut numX = 0;
            let mut numO = 0;
            let mut numB = 0;

            for i in 0..k {
                if s[i][j] == 'x' {
                    numX += 1;
                } else if s[i][j] == 'o' {
                    numO += 1;
                } else {
                    numB += 1;
                }
            }
            if numX == 0 {
                let current_cost = numB;
                min_cost = min_cost.min(current_cost);
                found = true;
            }
            for i in 0..(h-k) {
                // We will add the char at i+k and remove the one at i
                if s[i][j] == 'x' {
                    numX -= 1;
                } else if s[i][j] == 'o' {
                    numO -= 1;
                } else {
                    numB -= 1;
                }
                if s[i+k][j] == 'x' {
                    numX += 1;
                } else if s[i+k][j] == 'o' {
                    numO += 1;
                } else {
                    numB += 1;
                }
                if numX == 0 {
                    let current_cost = numB;
                    min_cost = min_cost.min(current_cost);
                    found = true;
                }
            }
        }
    }

    if !found {
        println!("-1");
        return;
    }
    println!("{}", min_cost);

}
