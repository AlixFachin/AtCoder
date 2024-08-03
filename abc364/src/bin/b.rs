use std::hash;
use proconio::marker::Chars;
use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
        h: usize,
        w: usize,
        start_i: usize,
        start_j: usize,
        c: [Chars;h],
        x: Chars,
    };

    let mut pos_row = start_i-1;
    let mut pos_col = start_j-1;

    for move_t in x {
        if DEBUG {
            println!("pos_row: {}, pos_col: {}", pos_row, pos_col);
        }
        match move_t {
            'U' => {
                if pos_row == 0 {
                    continue;
                }
                if c[pos_row-1][pos_col] == '.' {
                    pos_row -= 1;
                }
            },
            'D' => {
                if pos_row == h-1 {
                    continue;
                }
                if c[pos_row+1][pos_col] == '.' {
                    pos_row += 1;
                }
            },
            'L' => {
                if pos_col == 0 {
                    continue;
                }
                if c[pos_row][pos_col-1] == '.' {
                    pos_col -= 1;
                }
            },
            'R' => {
                if pos_col == w-1 {
                    continue;
                }
                if c[pos_row][pos_col+1] == '.' {
                    pos_col += 1;
                }
            },
            _ => {
                panic!("unexpected move_t: {}", move_t);
            },
        }
    }

    println!("{} {}", pos_row+1, pos_col+1);

}
