use proconio::input;

fn compute(d: i64, x: i64, y: i64) -> i64 {
    // Implementing my own version of the function to minimize
    // in order to keep everything with the same i64 type

    let a = x*x + y*y;
    if d > a {
        return d - a;
    }
    return a - d;

}

fn find_min(d: i64) -> i64 {
    match d {
        1 => 0,
        2 => 0,
        3 => 0,
        4 => 0,        
        _ => {
            
            let s = (d as f64).sqrt().ceil() as i64;
            let mut min_f = d;
            for x in 1..=s {

                    if x*x > d {
                        let f = compute(d, x, 0);
                        if f < min_f {
                            min_f = f;
                        }
                    } else {
                        let delta: i64 = d.abs_diff(x*x) as i64;
                        let y1 = (delta as f64).sqrt().floor() as i64;
                        let y2 = (delta as f64).sqrt().ceil() as i64;

                        let f1 = compute(d, x, y1);
                        let f2 = compute(d,x,y2);
                        min_f = min_f.min(f1);
                        min_f = min_f.min(f2);

                    }
            }

            return min_f as i64;

        }
    }
}

fn main() {
    input! {
        d: i64,
    }

    let result = find_min(d);

    println!("{}",result);

}
