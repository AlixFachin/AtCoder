use proconio::input;
use num_integer::{cbrt, Roots};

fn is_palindrome(x: usize) -> bool {
    let s = x.to_string();
    let r = s.chars().rev().collect::<String>();
    return s == r
}

#[test]
fn test_palindrome(){
    assert_eq!(is_palindrome(8), true);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(122), false);
    assert_eq!(is_palindrome(1221), true);
    assert_eq!(is_palindrome(1231343434), false);
    assert_eq!(is_palindrome(13349966994331), true);
}

fn get_cubic_root(x: usize) -> usize {
    let a = x.cbrt();
    if (a*a*a) > x {
        return a - 1;
    }
    if (a*a*a) <=  x && ((a+1)*(a+1)*(a+1)) > x {
        return a;
    }
    return a;
}

fn get_cubic_root_newton(x: usize) -> usize {
    let mut low: usize = 0;
    let mut high: usize = 3_000_000;
    let mut found: bool = false;

    if x == 0 {
        return 0;
    }

    while !found {
        if (high - low <= 1 && (low*low*low) <= x && ((low+1)*(low+1)*(low+1)) > x) {
            found = true;
        } else {
            let mid = (low + high) / 2;
            if (mid*mid*mid) <= x {
                low = mid;
            } else {
                high = mid;
            }
        }
    }
    return low;
}

#[test]
fn test_newton() {
    assert_eq!(get_cubic_root_newton(8), 2);
    assert_eq!(get_cubic_root_newton(9), 2);
    assert_eq!(get_cubic_root_newton(27), 3);
    assert_eq!(get_cubic_root_newton(30), 3);
    assert_eq!(get_cubic_root_newton(475_382*475_382*475_382 - 1), 475_381);
    assert_eq!(get_cubic_root_newton(107430649247442965), 475_381);
    assert_eq!(get_cubic_root_newton(107430649247442964), 475_381);
    assert_eq!(get_cubic_root_newton(107430649247442968), 475_382);
    assert_eq!(get_cubic_root_newton(107430649247442969), 475_382);
    assert_eq!(get_cubic_root_newton(107430649247442970), 475_382);
    assert_eq!(get_cubic_root_newton(107430649247443453), 475_382);

}

#[test]
fn test_cubic_root() {
    assert_eq!(get_cubic_root(8), 2);
    assert_eq!(get_cubic_root(9), 2);
    assert_eq!(get_cubic_root(27), 3);
    assert_eq!(get_cubic_root(30), 3);
    assert_eq!(get_cubic_root(475_382*475_382*475_382 - 1), 475_381);
    assert_eq!(get_cubic_root(107430649247442965), 475_381);
    assert_eq!(get_cubic_root(107430649247442964), 475_381);
    assert_eq!(get_cubic_root(107430649247442968), 475_382);
    assert_eq!(get_cubic_root(107430649247442969), 475_382);
    assert_eq!(get_cubic_root(107430649247442970), 475_382);
    assert_eq!(get_cubic_root(107430649247443453), 475_382);

}

fn main() {
    input! {
       n: usize,
    };

    if n == 0 {
        println!("{}", n);
        return;
    }

    let mut x = get_cubic_root_newton(n);
    while !is_palindrome(x*x*x) {
        x -= 1;
    }
    println!("{}", x*x*x);

}
