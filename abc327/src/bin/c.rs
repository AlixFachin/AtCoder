use proconio::input;

fn check_row(grid: &Vec<Vec<i32>>,row: usize ) -> bool {

    let mut result: Vec<i32> = vec![];
    for i in 0..9 {
        result.push(grid[row][i]);
    }
    result.sort();
    return result == vec![1,2,3,4,5,6,7,8,9];

}

#[test]
fn test_check_row() {
    let v = vec![vec![1,2,3,4,5,6,7,8,9], vec![4,5,6,7,8,9,1,2,3], vec![7,8,9,1,2,3,4,5,6], vec![2,3,4,5,6,7,8,9,1], vec![5,6,7,8,9,1,2,4,4],
        vec![8,9,1,2,3,4,5,6,7],vec![3,4,5,6,7,8,9,1,2],vec![6,7,8,9,1,2,3,4,5],vec![9,1,2,3,4,5,6,7,8]];

    assert!(check_row(&v, 0));
    assert!(check_row(&v, 1));
    assert!(check_row(&v, 2));
    assert!(!check_row(&v, 4));

    assert!(check_col(&v, 0));

}

fn check_col(grid: &Vec<Vec<i32>>,col: usize ) -> bool {

    let mut result: Vec<i32> = vec![];
    for i in 0..9 {
        result.push(grid[i][col]);
    }
    result.sort();
    return result == vec![1,2,3,4,5,6,7,8,9];

}

fn check_subgrid(grid: &Vec<Vec<i32>>, index: usize) -> bool {
    let row_subgrid_center = index / 3;
    let col_subgrid_center = index %3;

    let mut result: Vec<i32> = vec![];

    for i in 0..3 {
        for j in 0..3 {
            result.push(grid[row_subgrid_center*3+i][col_subgrid_center*3+j]);
        }
    }
    result.sort();
    return result == vec![1,2,3,4,5,6,7,8,9];

}


fn main() {
    input! {
        grid: [[i32;9];9],
    }

    for i in 0..9 {
        if !check_row(&grid, i) {
            println!("No");
            return;
        }
        if !check_col(&grid, i) {
            println!("No");
            return;
        }
        if !check_subgrid(&grid, i) {
            println!("No");
            return;
        }
    }    
    println!("Yes");

}
