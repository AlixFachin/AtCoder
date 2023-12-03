use proconio::input;

fn main() {
    input! {
       n : usize,
       s: usize, // s: price for 6
       m: usize, // m: price for 8
       l: usize, // l: price for 12
    }

    let mut min_price = 100_000;
    for q_s in 0..20 {
        for q_m in 0..15 {
            for q_l in 0..10 {
                let qty_bought = q_s*6 + q_m * 8 + q_l * 12;
                let price_bought = q_s * s + q_m * m + q_l * l;
                if qty_bought >= n && price_bought <= min_price {
                    min_price = price_bought;
                }


            }
        }
    }

    println!("{}", min_price);

}
