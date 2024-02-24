use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn get_next_move(row: usize, col: usize, delta: char, h: &usize, w: &usize) -> (usize, usize) {
    match delta {
        'U' => {
            return (row.max(1) - 1, col);
        }
        'D' => {
            return (row.min(*h - 2) + 1, col);
        }
        'L' => {
            return (row, col.max(1) - 1);
        }
        'R' => {
            return (row, col.min(*w - 2) + 1);
        }

        _ => panic!("Invalid delta: {}", delta),
    }
}

fn is_path_possible(
    grid: &Vec<Vec<char>>,
    path: &Vec<char>,
    path_index: usize,
    row: usize,
    col: usize,
    h: &usize,
    w: &usize,
) -> bool {
    if path_index > path.len() {
        panic!("Invalid path index: {}", path_index);
    }

    if path_index == path.len() {
        return grid[row][col] == '.';
    }

    let is_current_land = grid[row][col] == '.';
    if !is_current_land {
        return false;
    }

    let (next_row, next_col) = get_next_move(row, col, path[path_index], h, w);

    return is_path_possible(grid, path, path_index+1, next_row, next_col, h, w);
}

fn main() {
    input! {
       h: usize,
       w: usize,
       n: usize,
       t: Chars,
       s: [Chars; h],
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.';w];h];
    for i in 0..h {
        for j in 0..w {
            grid[i][j] = s[i][j];
        }
    }
    if DEBUG {
        println!("Initial grid:");
        for i in 0..h {
            for j in 0..w {
                print!("{}", grid[i][j]);
            }
            println!("");
        }
    }

    let mut count_possible_start = 0;
    for row in 1..(h-1) {
        for col in 1..(w-1) {
            if grid[row][col] == '.' {
                let is_possible = is_path_possible(&grid, &t, 0, row, col, &h, &w);
                if is_possible {
                    count_possible_start += 1;
                }
            }
        }
    }

    println!("{}", count_possible_start);


}
