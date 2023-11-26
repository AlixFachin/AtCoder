use proconio::input;

fn get_divisor(n: i32, i: i32) -> String {

    for j in 1..=9 {
        if n % j == 0 && i % (n/j) == 0 {
            return j.to_string()
        }
    }

    return "-".to_string()
}

#[test]
fn test_divisor() {

    assert_eq!(get_divisor(12,0),"1");
    assert_eq!(get_divisor(12,1),"-");
    assert_eq!(get_divisor(12,2),"6");
    assert_eq!(get_divisor(12,12),"1");
    
    assert_eq!(get_divisor(11,0),"1");
    assert_eq!(get_divisor(11,1),"-");
    assert_eq!(get_divisor(11,2),"-");




}

fn main() {

    input! {
        n: i32,
    }    

    let mut s: String = "".to_string();

    for i in 0..=n {
        s = s + get_divisor(n, i).as_str();
    }

    println!("{}",s);
}
