use proconio::input;

fn two_to_the_n(n: usize) -> usize {

    let mut result: usize = 1;
    for _i in 0..n {
        result = result *2;
    }
    return result;
}

#[test]
fn test_two() {
    assert_eq!(two_to_the_n(0),1);
    assert_eq!(two_to_the_n(1),2);
    assert_eq!(two_to_the_n(2),4);
    assert_eq!(two_to_the_n(3),8);
    assert_eq!(two_to_the_n(4),16);
}

fn main() {
    input! {
        n: usize,
        s:[String;n], 
    }

    let mut cur_count: usize = 1;
    for i in 0..n {
        if s[i] == "OR" {
            cur_count = two_to_the_n(i+1) + cur_count;
        }
    }

    println!("{}", cur_count);
}
