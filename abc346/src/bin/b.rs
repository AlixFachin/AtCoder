use proconio::input;

const DEBUG:bool = false;

fn count_chars(s: &str) -> (usize, usize) {
    let mut w = 0;
    let mut b = 0;
    for c in s.chars() {
        if c == 'w' {
            w += 1;
        } else {
            b += 1;
        }
    }
    return (w, b)
}

#[test]
fn test_count() {
    assert_eq!(count_chars("wbwbwwbwbwbw"), (7, 5));
    assert_eq!(count_chars("wbwbwwbwbwbwbwbwbwwbwbwbw"), (14, 11));
}

fn main() {
    input! {
       w: usize,
       b: usize,
    };

    let pattern = "wbwbwwbwbwbw";
    let mut piano = String::new();
    for _i in 0..100 {
        piano.push_str(pattern);
    }

    // Let's brute force the issue
    let s_len = w+b;
    for i in 0..(piano.len()-s_len) {
        let s = &piano[i..i+s_len];
        let (w_count, b_count) = count_chars(s);
        if w_count == w && b_count == b {
            println!("Yes");
            return;
        }
    }
    println!("No");

}
