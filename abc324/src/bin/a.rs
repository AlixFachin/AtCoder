use proconio::input;

fn main() {
    
    input! {
        n: usize,
        values: [i32; n],
    }

    let mut all_equal = true;
    for i in 0..n {
        if values[i] != values[0] {
            all_equal = false;
            break;
        }
    }
    if all_equal {
        println!("Yes");
    } else {
        println!("No");
    }




}
