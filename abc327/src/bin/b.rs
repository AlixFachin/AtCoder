use proconio::input;

const DEBUG : bool = false;

fn mypow(x:u64, n: u64) -> u64 {
    let mut result:u64 = 1;
    for i in 1..=n {
        result = result * x;
    }
    return result;
}

#[test] 
fn test_mypow(){

    assert_eq!(mypow(2, 2),4);
    assert_eq!(mypow(3, 3),27);
    assert_eq!(mypow(4, 4),4u64.pow(4));
    assert_eq!(mypow(5, 5),5u64.pow(5));
    assert_eq!(mypow(6, 6),6u64.pow(6));
    assert_eq!(mypow(7, 7),7u64.pow(7));
    assert_eq!(mypow(8, 8),8u64.pow(8));
    assert_eq!(mypow(9, 9),9u64.pow(9));
    assert_eq!(mypow(10, 10),10u64.pow(10));
    assert_eq!(mypow(11, 11),11u64.pow(11));
    assert_eq!(mypow(12, 12),12u64.pow(12));
    assert_eq!(mypow(13, 13),13u64.pow(13));
    assert_eq!(mypow(14, 14),14u64.pow(14));
    assert_eq!(mypow(15, 15),15u64.pow(15));

}

fn main() {
    
    input!{
        b: u64,
    }

    for a in 1..16u64 {
        if DEBUG {
            println!("{}pow{} is {}",a,a, a.pow(a as u32));
        }

        if mypow(a,a) == b {
            println!("{}",a);
            return;
        }

    }

    println!("-1");
}

// 437893890380859375
// 1000000000000000000
// 1152921504606846976
