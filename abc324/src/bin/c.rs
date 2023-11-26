
use proconio::input;

fn has_one_difference_or_less(a: &str, b: &str) -> bool {
    let pair_iter = a.chars().zip(b.chars());
    let mut num_diff = 0;
    for (a_char, b_char) in pair_iter {
        if a_char != b_char {
            num_diff = num_diff + 1;
        }
        if num_diff >= 2 {return false;}
    }
    return true;
}


fn has_one_inserted(long: &str, short: &str) -> bool {
    // We assume that long is longer than short
    if long.len() - short.len() != 1 {
        panic!("Couille dans le potage");
    }

    let long_vec: Vec<char> = long.chars().collect();
    let short_vec: Vec<char> = short.chars().collect();
    // First block - similar
    let mut i :usize =0;
    while i < short_vec.len() && long_vec[i] == short_vec[i] {
        i = i + 1;
    }
    // Now we have the difference
    if i == short_vec.len() { return true;}
    // We skip the difference - then all chars should be equal
    while i < short_vec.len() {
        if short_vec[i] != long_vec[i+1] { return false;}
        i = i+1;
    }

    return true;
}

fn is_potential(message: &str, original: &str) -> bool {

    if message == original {
        return true;
    }

    // Second case -> same length. Then number of differences should be at least one
    if message.len() == original.len() {
        return has_one_difference_or_less(message, original);
    }

    if message.len() > original.len()  {
        if message.len() - original.len() >= 2 {
            return false;
        }
        return has_one_inserted(message, original)
    }

    if message.len() < original.len()  {
        if original.len() - message.len() >= 2 {
            return false;
        }
        return has_one_inserted(original, message);
    }

    panic!("Case is supposed to be never reached");

}
fn main() {

    input! {
        n: usize,
        aoki: String,
        potentials: [String; n],
    }

    let mut good_strings: Vec<i32> = Vec::new();
    let mut i = 1;

    for s in potentials {

        if is_potential(&s, &aoki) {
            good_strings.push(i);
        }
        i = i+1;
    }

    println!("{}",good_strings.len());
    for x in good_strings {
        print!("{} ", x);
    }
    println!("{}","");

}
