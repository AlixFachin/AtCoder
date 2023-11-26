use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }

    // Other algorithms: look at how many mutliples of c and multiples of d
    let mut c_multiples : Vec<u64> = vec![];
    let mut d_multiples: Vec<u64> = vec![];
    let mut c_mult_count: u64 = 0;
    let mut d_mult_count: u64 = 0;
    let mut cd_mult_count: u64 = 0;

    let remainder = (a-1) % c;
    let mut x:u64 = a - 1 + c - remainder; 
    while x <= b {
        c_multiples.push(x);
        c_mult_count = c_mult_count +1;
        if x %d == 0 {
            cd_mult_count = cd_mult_count + 1;
        }
        x = x + c;
    }

    if DEBUG {
        let remainder = (a-1) % d;
        let mut x:u64 = a - 1 + d - remainder; 
        while x <= b {
            d_multiples.push(x);        
            x = x + d;
        }
    }


    d_mult_count = b / d - a / d;

    if DEBUG {
        println!("Multiples of {} are {:?}",c, c_multiples);
        println!("Multiples of {} are {:?}",d, d_multiples);
        println!("Count of multiples of d: {}", d_mult_count);
    }

    // Now we have to figure out which numbers are both multiples of b and d
    // let mut common_multiples: u64 = 0;
    // for mult_c in c_multiples {
    //     if d_multiples.binary_search(&mult_c).is_ok() {
    //         common_multiples = common_multiples + 1;
    //     }
    // }

    let result = b - a + 1 + cd_mult_count - c_mult_count - d_mult_count;

    println!("{}",result);

}
