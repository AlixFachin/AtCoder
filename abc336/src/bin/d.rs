use proconio::input;
const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       a: [usize; n],
    }

    if n == 1 {
        println!("1");
        return;
    }

    // Find the size of the biggest pyramid possible to be built with the sequence a
    // 1. We can only remove items, so the pyramid size is capped at (n+1)/2
    // 2. We can only subtract 1 from values inside the pyramid, so for the pyramid to be valid, it has

    let mut max_sequence_size: Vec<usize> = vec![0;n];
    let mut left_max = vec![0;n];
    let mut right_max = vec![0;n];
    for i in 0..n {
        left_max[i] = a[i].min(i + 1);
        if i > 0 {
            left_max[i] = left_max[i].min(left_max[i - 1] + 1);
        }
    }
    for i in (0..n).rev() {
        right_max[i] = a[i].min(n - i);
        if i < n - 1 {
            right_max[i] = right_max[i].min(right_max[i + 1] + 1);
        }
        max_sequence_size[i] = left_max[i].min(right_max[i]);
    }
    if DEBUG {
        println!("{:?}", left_max);
        println!("{:?}", right_max);
        println!("{:?}", max_sequence_size);
    }

    let ans = max_sequence_size.iter().max().unwrap();
    println!("{}", ans);
}
