use proconio::input;

const DEBUG : bool = false;

struct BottleInfo {
    capacity: f64,
    price: i32,
    std_price: f64,
}

fn main() {
    input! {
        q: i32,
        h: i32,
        s: i32,
        d: i32,
        n: i32,
    }

    let mut bottles: Vec<BottleInfo> = vec![ 
            BottleInfo { capacity: 0.25, price: q, std_price: (q * 4) as f64 },
            BottleInfo { capacity: 0.5, price: h, std_price: (h * 2) as f64 },
            BottleInfo { capacity: 1.0, price: s, std_price: s as f64 },
            BottleInfo { capacity: 2.0, price: d, std_price: (d as f64) / 2.0 } ];

    bottles.sort_by(|b1,b2 | (b1.std_price).total_cmp(&b2.std_price));

    if DEBUG {
        for b in &bottles {
            println!("Bottle {} has stdPrice of {} and price of {}", b.capacity, b.std_price, b.price);
        }
    }
    
    let mut remainder = n as f64;
    let mut cost = 0.0;

    for b in bottles {
        let qty_bottles = (remainder / b.capacity).floor();
        
        cost = cost + qty_bottles * (b.price as f64);
        remainder = remainder - (qty_bottles * b.capacity); 

        if DEBUG {
            println!("Using {} qty of bottles {} for a price of {}", qty_bottles, b.capacity, qty_bottles * (b.price as f64) );
        }

    }

    println!("{}",cost as u64);

}
