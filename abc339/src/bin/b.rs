use proconio::input;

const DEBUG: bool = false;

fn turn_clockwise(dir: (isize, isize)) -> (isize, isize) {
    return match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn turn_counter_clockwise(dir: (isize, isize)) -> (isize, isize) {
    return match dir {
        (-1, 0) => (0, -1),
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    input! {
       h: usize,
       w: usize,
       n: usize,
    };

    let mut grid = vec![vec!['.'; w]; h];
    let mut curr : (usize, usize) = (0, 0);
    let mut dir: (isize, isize) = (-1, 0);

    for step in 0..n {

        if grid[curr.0][curr.1] == '.' {
            // White cell: We paint in black, then turn 90 deg clocwise and go fwd
            grid[curr.0][curr.1] = '#';
            dir = turn_clockwise(dir);
            curr.0 = (h as isize + curr.0 as isize + dir.0) as usize % h;
            curr.1 = (w as isize + curr.1 as isize + dir.1) as usize % w;
        } else {
            // Black cell: We paint in white, then turn 90 deg counter-clockwise and go fwd
            grid[curr.0][curr.1] = '.';
            dir = turn_counter_clockwise(dir);
            curr.0 = (h as isize + curr.0 as isize + dir.0) as usize % h;
            curr.1 = (w as isize + curr.1 as isize + dir.1) as usize % w;
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", grid[i][j]);
        }
        println!("");
    }

}
