use proconio::input;


fn diff_steps(a: f64, b: f64, k: f64) -> f64 {
    return b - a * ( 1./(1.+k).sqrt() - 1./(2.+k).sqrt());
}
 
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    // let mut min_time: f64 = a;
    // for k in 0..100_000 {
    //     let cur_time = b * (k as f64) + a / ((1.0 + k as f64).sqrt());
    //     min_time = min_time.min(cur_time);
    // }

    // println!("{}", min_time);

    let mut x1: usize = 0;
    let mut x2: usize = 10_000;

    while (x2 - x1 > 1) {

        let x1_f = x1 as f64;
        let x2_f = x2 as f64;

        if diff_steps(a,b,x2_f) < 0. {
            x1 = x2;
            x2 = x1 * 2;
        } else {
            let middle = (x1_f + x2_f)/2.;
            if diff_steps(a, b, middle) < 0.0 {
                x2 = x1 + (x2 - x1)/2;
            } else {
                x1 = x1 + (x2 - x1)/2;
            }
        }

    }

}
