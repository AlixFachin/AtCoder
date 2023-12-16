use proconio::input;

const DEBUG: bool = false;

fn get_corresponding_row(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, a_row_index: usize, b_rows_already_used: &Vec<Option<usize>>) -> Option<usize> {

    let mut a_row_sorted = a[a_row_index].clone();
    a_row_sorted.sort();

    for b_i in 0..b.len() {
        if b_rows_already_used[b_i].is_none() {
            let mut b_row_sorted = b[b_i].clone();
            b_row_sorted.sort();

            let mut has_diff = false;
            for j in 0..b_row_sorted.len() {
                if a_row_sorted[j] != b_row_sorted[j] {
                    has_diff = true;
                }
            }
            if !has_diff {
                return Some(b_i);
            }
        }
    }
    return None;
} 

fn get_corresponding_col(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, a_col_index: usize, b_cols_already_used: &Vec<Option<usize>>) -> Option<usize> {

    let mut a_col_sorted: Vec<usize> = a.iter().map(|a_row| a_row[a_col_index]).collect();    
    a_col_sorted.sort();

    for b_j in 0..b[0].len() {
        if b_cols_already_used[b_j].is_none() {
            let mut b_col_sorted: Vec<usize> = b.iter().map(|b_row| b_row[b_j]).collect();
            b_col_sorted.sort();
            let mut has_diff = false;
            for i in 0..b_col_sorted.len() {
                if a_col_sorted[i] != b_col_sorted[i] {
                    has_diff = true;
                }
            }
            if !has_diff {
                return Some(b_j);
            }
        }
    }
    return None;
}

fn main() {
    input! {
       h: usize,
       w: usize,
       a: [[usize;w];h],
       b: [[usize;w];h],
    };
    
    if DEBUG {
        println!("Matrix a: \n {:?}", a);
        println!("Matrix b: \n {:?}", b);
    }

    // corresponding_row will contain the index of a corresponding to a given index of b
    let mut corresponding_rows : Vec<Option<usize>> = vec![None;h];
    // First, the conditions: every row of a has to be a combination of one row of b
    for a_row in 0..h {
        
        match get_corresponding_row(&a, &b, a_row, &corresponding_rows) {
            None => {
                println!("-1");
                return;
            }
            Some(corresponding_b) => {
                corresponding_rows[corresponding_b] = Some(a_row);
            }
        }
    }

    let mut corresponding_columns: Vec<Option<usize>> = vec![None;w];
    for a_col in 0..w {

        match get_corresponding_col(&a, &b, a_col, &corresponding_columns) {
            None => {
                println!("-1");
                return;
            }
            Some(corresponding_b) => {
                corresponding_columns[corresponding_b] = Some(a_col);
            }
        }
    }

    if DEBUG {
        println!("Corresponding rows: {:?}", corresponding_rows);
        println!("Corresponding columns: {:?}", corresponding_columns);
    }

    // Counting number of different rows
    let mut swap_rows: Vec<usize> = corresponding_rows.iter().map(|x| x.unwrap()).collect();
    let mut swap_cols: Vec<usize> = corresponding_columns.iter().map(|x| x.unwrap()).collect();

    let mut count_operations: usize = 0;
    // Sorting the array by bubble sort, counting the operations
    for _i in 0..swap_rows.len() {
        for j in 0..swap_rows.len()-1 {
            if swap_rows[j] > swap_rows[j+1] {
                let temp = swap_rows[j];
                swap_rows[j] = swap_rows[j+1];
                swap_rows[j+1] = temp;
                count_operations = count_operations +1;
            }
        }
    }

    for _i in 0..swap_cols.len() {
        for j in 0..swap_cols.len()-1 {
            if swap_cols[j] > swap_cols[j+1] {
                let temp = swap_cols[j];
                swap_cols[j] = swap_cols[j+1];
                swap_cols[j+1] = temp;
                count_operations = count_operations +1;
            }
        }
    }

    println!("{}", count_operations);

}
