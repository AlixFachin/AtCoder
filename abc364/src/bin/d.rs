use std::cmp::Ordering;
use superslice::*;
use proconio::input;

const DEBUG:bool = false;


fn count_in_interval(a: &Vec<i64>, b: i64, x: i64) -> usize {
    
    if DEBUG { println!("a: {:?}", *a) }
    
    let lbound_index = a.lower_bound(&(b - x));
    if DEBUG { println!("lbound_index: {}", lbound_index) }

    let ubound_index = a.upper_bound(&(x + b));
    if DEBUG { println!("ubound_index: {}", lbound_index) }


    return ubound_index - lbound_index;
}

#[test]
fn test_count_in_interval() {
    let a = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(count_in_interval(&a, 5, 1), 3);
    assert_eq!(count_in_interval(&a, 5, 2), 5);
    assert_eq!(count_in_interval(&a, 5, 3), 7);
    assert_eq!(count_in_interval(&a, 5, 4), 9);
    assert_eq!(count_in_interval(&a, 5, 5), 10);
    assert_eq!(count_in_interval(&a, 5, 6), 10);
    assert_eq!(count_in_interval(&a, 5, 7), 10);
    assert_eq!(count_in_interval(&a, 5, 8), 10);
    assert_eq!(count_in_interval(&a, 5, 9), 10);
    assert_eq!(count_in_interval(&a, 5, 10), 10);
    assert_eq!(count_in_interval(&a, 5, 11), 10);
}


fn main() {
    input! {
       n: usize,
       q: usize,
       mut a: [i64;n],
       quest: [(i64, usize); q],
    };

    a.sort();
    for (b,k) in quest {
        // We will look to count how many elements of a are between b-x and b+x
        let mut l: i64 = -1;
        let mut r: i64 = 1e9 as i64;
        while r - l > 1 {
            let mid = (l+r)/2;
            if count_in_interval(&a, b, mid) >= k {
                r = mid;
            } else {
                l = mid;
            }
        }
        println!("{}", r);
    }


}
