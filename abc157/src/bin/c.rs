use proconio::input;

fn char_to_usize(c: &char) -> usize {

    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Wrong number"),
    }

}

fn main() {

    input! {
        n: usize,
        m: usize,
        fixed: [(usize, usize); m],
    }

    // let mut result: Vec<i32> = vec![-1;n];

    // for (s,c) in fixed {
    //     if result[s-1] != -1 && result[s-1] != c as i32 {
    //         // We are trying to write twice the same number
    //         println!("-1");
    //         return;
    //     }
    //     result[s-1] = c as i32;
    // }

    // if result[0] == 0 && n > 1 {
    //     println!("-1");
    //     return;
    // }
    // if result[0] == -1 {
    //     result[0] = 1;
    // }
    // for i in 1..n {
    //     if result[i] == -1 {
    //         result[i] = 0;
    //     }
    // }
    // for i in 0..n {
    //     print!("{}", result[i]);
    // }
    // println!("");

    for x in 0..=999 {
        let x_s = x.to_string();
        if x_s.len() != n {
            continue;
        }

        let mut correct = true;
        for (s_j, c_j) in &fixed {
            if char_to_usize(&x_s.chars().nth(s_j-1).unwrap()) != *c_j {
                correct = false;
            }

        }
        if correct {
            println!("{}",x);
            return;
        }
    }
    println!("-1");

}
