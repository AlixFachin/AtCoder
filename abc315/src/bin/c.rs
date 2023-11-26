use proconio::input;

fn main() {
    input!{
        n: usize,
        values:[(usize, usize);n],
    }

    let mut sorted =  values.clone();
    sorted.sort_by(| (a1,b1) , (a2,b2) | b2.cmp(b1));

    let mut del = 0;
    let (best_a, best_b) = sorted[0];

    for i in 1..sorted.len() {
        let del_temp: usize;
        let (a,b) = sorted[i];
        if best_a == a {
            del_temp = best_b + b / 2;
        } else {
            del_temp = best_b + b;
        }
        if del_temp > del {
            del = del_temp;
        }
    }

    println!("{}",del);

}
