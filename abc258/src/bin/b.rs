use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn get_number_on_matrix(a: &Vec<Vec<u8>>, i: &usize, j: &usize, n: &u8, direction: &str) -> usize {

    let mut delta_i: i8 = 0;
    let mut delta_j: i8 = 0;
    match direction {
        "N" => { delta_i = -1; },
        "S" => { delta_i = 1; },
        "E" => { delta_j = 1; },
        "W" => { delta_j = -1; },
        "NE" => { delta_i = -1; delta_j = 1; },
        "NW" => { delta_i = -1; delta_j = -1; },
        "SE" => { delta_i = 1; delta_j = 1; },
        "SW" => { delta_i = 1; delta_j = -1; },
        _ => panic!("Invalid direction ${direction}"),
    }

    let mut result = 0;
    for k in 0..*n {
        let current_i = ((*n as i8 + *i as i8 + delta_i * k as i8) % (*n as i8)) as usize;
        let current_j = ((*n as i8 + *j as i8 + delta_j * k as i8) % (*n as i8)) as usize;
        result = result * 10 + a[current_i][current_j] as usize;
    }
    return result;
}


fn main() {
    input! {
       n: u8,
       a0: [Chars;n],
    };

    let mut a: Vec<Vec<u8>> = vec![vec![0 as u8;n as usize];n as usize];
    for i in 0..n {
        for j in 0..n {
            a[i as usize][j as usize] = a0[i as usize][j as usize] as u8 - '0' as u8;
        }
    }

    // Looking at the maximum value of a
    let mut m: u8 = 0;
    a.iter().for_each(|x| {
        x.iter().for_each(|y| {
            m = m.max(*y);
        });
        println!();
    });

    let mut optimal_start_positions = vec![];
    a.iter().enumerate().for_each(|(i, x)| {
        x.iter().enumerate().for_each(|(j, y)| {
            if *y == m {
                optimal_start_positions.push((i, j));
            }
        });
    });

    let mut max_number: usize = 0;
    for (i_s,j_s) in optimal_start_positions {
        if DEBUG { println!("Checking start position: {i_s}, {j_s}")}
        for d in ["N", "S", "E", "W", "NE", "NW", "SE", "SW"].iter() {
            let number = get_number_on_matrix(&a, &i_s, &j_s, &n, d);
            if DEBUG { println!("   Number found: {number} for direction {d}")}
            max_number = max_number.max(number);
        }
    }

    println!("{}", max_number);

}
