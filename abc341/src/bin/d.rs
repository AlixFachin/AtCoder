use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       m: usize,
       k: usize,
    };

    let a: usize = n.min(m);
    let b:usize = n.max(m);

    let p = a*b;
    let mut multiples : Vec<usize> = vec![];

    for i in 1..b {
        multiples.push( a*i);
    }
    for i in 1..a {
        multiples.push( b*i);
    }
    multiples.sort();
    
    let offset = (k-1) / (a+b-2);
    let remainder = (k-1) % (a + b - 2);
    if DEBUG {
        println!("A: {}, B: {}, P: {}", a, b, p);
        println!("Multiples: {:?}", multiples);
        println!("Offset: {}, Remainder: {}", offset, remainder);
    }
    
    let res = offset * p + multiples[remainder];

    println!("{}", res);

    // let mut a_mult = 1;
    // let mut b_mult = 1;
    // let mut count = 0;
    // let mut last_value : usize = 0;
    // while count < k {
    //     if DEBUG {
    //         println!("multiple of a: {}, multiple of b: {}, (a_mult: {}, b_mult: {}), count: {}", &(a * a_mult), b * b_mult, a_mult, b_mult, count);
    //     }
    //     if a * a_mult < b * b_mult {
    //         if DEBUG {
    //             println!("A is smaller - skipping to the next multiple of A")
    //         }
    //         last_value = a*a_mult;
    //         a_mult = a_mult + 1;
    //         while a_mult % b == 0 {
    //             a_mult = a_mult + 1;
    //         }
    //         count += 1;
    //         if DEBUG {
    //             println!("After increase: a_mult:{}, b_mult:{}, count:{}", a_mult, b_mult, count);
    //         }

    //     } else {
    //         if a == 1 {
    //             // We have to skip to the next multiple of B without counting this one
    //             b_mult = b_mult + 1;
    //         } else {
    //             last_value = b*b_mult;
    //             if DEBUG {
    //                 println!("B is smaller - skipping to the next multiple of B")
    //             }
    //             b_mult = b_mult + 1;
    //             while b_mult % a == 0 {
    //                 b_mult = b_mult + 1;
    //             }
    //             count += 1;
    //             if DEBUG {
    //                 println!("After increase: a_mult:{}, b_mult:{}, count:{}", a_mult, b_mult, count);
    //             }
    //         }
    //     }
    //     io::stdout().flush().unwrap();
    // }
    // println!("{}", last_value);
}
