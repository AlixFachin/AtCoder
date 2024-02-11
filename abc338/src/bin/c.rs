use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       q: [usize; n],
       a: [usize; n],
       b:  [usize; n],
    };

    let mut qty_a = vec![0; n];
    let mut min_qty_a = usize::MAX;
    let mut qty_b = vec![0; n];
    let mut min_qty_b = usize::MAX;
    for i in 0..n {
        if a[i] == 0 {
            qty_a[i] = usize::MAX;
        } else {
            qty_a[i] = q[i]/a[i];
        }
        min_qty_a = min_qty_a.min(qty_a[i]);
        if b[i] == 0 {
            qty_b[i] = usize::MAX;
        } else {
            qty_b[i] = q[i]/b[i];
        }
        min_qty_b = min_qty_b.min(qty_b[i]);
    }
    if DEBUG {
        println!("qty_a: {:?}", qty_a);
        println!("qty_b: {:?}", qty_b);
    }
    let max_a = qty_a.iter().min().unwrap();
    let max_b = qty_b.iter().min().unwrap();

    if DEBUG {
        println!("max_a: {:?}", max_a);
        println!("max_b: {:?}", max_b);
    }

    let mut remainders = vec![0; n];
    for i in 0..n {
        remainders[i] = q[i] - a[i]*max_a;
    }

    if DEBUG {
        println!("remainders: {:?}", remainders);
    }
    let mut max_servings:usize = 0;
    let mut alpha_a = *max_a;
    let mut alpha_b = remainders.iter().enumerate().map(|(i,r)| ( if b[i] == 0 { usize::MAX} else { r/b[i]} )).min().unwrap();
    max_servings = max_servings.max(alpha_a + alpha_b);

    while alpha_a  > 0 {
        if alpha_b == *max_b {
            // We reached the max - no need to go further
            break;
        }
        alpha_a -= 1;
        for i in 0..n {
            remainders[i] = remainders[i] + a[i];
        }
        alpha_b = remainders.iter().enumerate().map(|(i,r)| ( if b[i] == 0 { usize::MAX} else { r/b[i]} )).min().unwrap();
        max_servings = max_servings.max(alpha_a + alpha_b);
        if DEBUG {
            println!("alpha_a: {:?}", alpha_a);
            println!("alpha_b: {:?}", alpha_b);
            println!("remainders: {:?}", remainders);
            println!("max_servings: {:?}", max_servings);
        }
    }

    println!("{}", max_servings);

}
