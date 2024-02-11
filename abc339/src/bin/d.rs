use std::collections::{ HashMap, VecDeque };

use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn top(grid: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if x == 0 {
        return (x, y);
    }
    if grid[x - 1][y] == '.' {
        return (x - 1, y);
    }
    return (x, y);
}

fn bottom(grid: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if x == grid.len() - 1 {
        return (x, y);
    }
    if grid[x + 1][y] == '.' {
        return (x + 1, y);
    }
    return (x, y);
}

fn right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if y == grid.len() - 1 {
        return (x, y);
    }
    if grid[x][y + 1] == '.' {
        return (x, y + 1);
    }
    return (x, y);
}

fn left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, usize) {
    if y == 0 {
        return (x, y);
    }
    if grid[x][y - 1] == '.' {
        return (x, y - 1);
    }
    return (x, y);
}
fn main() {
    input! {
         n: usize,
         s: [Chars; n],
    }

    let mut grid = vec![vec!['.'; n]; n];
    let mut player_coord_1: (usize, usize) = (n + 1, 0);
    let mut player_coord_2: (usize, usize) = (n + 1, 0);
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                if player_coord_1.0 == n + 1 {
                    player_coord_1 = (i, j);
                } else {
                    player_coord_2 = (i, j);
                }
                grid[i][j] = '.';
            } else {
                grid[i][j] = s[i][j];
            }
        }
    }

    if DEBUG {
        println!("grid: {:?}", grid);
        println!("player_coord_1: {:?}", player_coord_1);
        println!("player_coord_2: {:?}", player_coord_2);
    }

    // We will doop a breadth-search for the solutions
    // So we will maintain a queue of possible moves

    // We will think about moving one player in 4-dimensional space (at least that's easier to think about for me)
    let mut coord = (player_coord_1.0, player_coord_1.1, player_coord_2.0, player_coord_2.1);
    // The queue will contain the coordinates + the number of moves it took to get there so far
    let mut queue: VecDeque<(usize, usize, usize, usize, usize)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
    visited.insert(coord, true);

    queue.push_back((coord.0, coord.1, coord.2, coord.3, 0));
    while queue.len() > 0 {
        let (x1, y1, x2, y2, moves) = queue.pop_front().unwrap();
        if DEBUG {
            println!("x1: {}, y1: {}, x2: {}, y2: {}, moves: {}", x1, y1, x2, y2, moves);
        }
        if x1 == x2 && y1 == y2 {
            println!("{}", moves);
            return;
        }
        // Let's check the top
        let (x1_top, y1_top) = top(&grid, x1, y1);
        let (x2_top, y2_top) = top(&grid, x2, y2);
        if (x1_top != x1 || y1_top != y1 || x2_top != x2 || y2_top != y2) && !visited.contains_key(&(x1_top, y1_top, x2_top, y2_top)) {
            visited.insert((x1_top, y1_top, x2_top, y2_top), true);
            queue.push_back((x1_top, y1_top, x2_top, y2_top, moves + 1));
        }
        if (x1_top == x2_top && y1_top == y2_top) {
            println!("{}", moves + 1);
            return;
        }
        // Let's check the bottom
        let (x1_bottom, y1_bottom) = bottom(&grid, x1, y1);
        let (x2_bottom, y2_bottom) = bottom(&grid, x2, y2);
        if (x1_bottom != x1 || y1_bottom != y1 || x2_bottom != x2 || y2_bottom != y2) && !visited.contains_key(&(x1_bottom, y1_bottom, x2_bottom, y2_bottom)) {
            queue.push_back((x1_bottom, y1_bottom, x2_bottom, y2_bottom, moves + 1));
            visited.insert((x1_bottom, y1_bottom, x2_bottom, y2_bottom), true);
        }
        if x1_bottom == x2_bottom && y1_bottom == y2_bottom {
            println!("{}", moves + 1);
            return;
        }
        // Let's check the right
        let (x1_right, y1_right) = right(&grid, x1, y1);
        let (x2_right, y2_right) = right(&grid, x2, y2);
        if (x1_right != x1 || y1_right != y1 || x2_right != x2 || y2_right != y2) && !visited.contains_key(&(x1_right, y1_right, x2_right, y2_right)) {
            queue.push_back((x1_right, y1_right, x2_right, y2_right, moves + 1));
            visited.insert((x1_right, y1_right, x2_right, y2_right), true);
        }
        if x1_right == x2_right && y1_right == y2_right {
            println!("{}", moves + 1);
            return;
        }
        // Let's check the left
        let (x1_left, y1_left) = left(&grid, x1, y1);
        let (x2_left, y2_left) = left(&grid, x2, y2);
        if (x1_left != x1 || y1_left != y1 || x2_left != x2 || y2_left != y2) && !visited.contains_key(&(x1_left, y1_left, x2_left, y2_left)) {
            queue.push_back((x1_left, y1_left, x2_left, y2_left, moves + 1));
            visited.insert((x1_left, y1_left, x2_left, y2_left), true);
        }
        if x1_left == x2_left && y1_left == y2_left {
            println!("{}", moves + 1);
            return;
        }
    }
    println!("-1");


}
