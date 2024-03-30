use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    };

    let mut first_letter_index: usize = 0;
    for (t, x) in queries {
        if t == 1 {
            if DEBUG { println!("first_letter_index: {}, x: {}", first_letter_index, x);}
            first_letter_index = (2*n + first_letter_index - x) %n;
        } else {
            if DEBUG { println!("Print query with {x}th letter, first_letter_index: {first_letter_index}")}
            println!("{}", s[(first_letter_index+(x-1))%n]);
        }
    }

}
