use proconio::input;

fn main() {
    input! {
       l: usize,
    };

    // QUESTION: How to avoid overflow?
    // One answer is with Pascal's triangle
    // The other answer is with dividing as soon as possible.

    let mut ans: usize = (l-1) * (l-2) / 2;
    ans = ans * (l-3) / 3;
    ans = ans * (l-4) / 4;
    ans = ans * (l-5) / 5;
    ans = ans * (l-6) / 6;
    ans = ans * (l-7) / 7;
    ans = ans * (l-8) / 8;
    ans = ans * (l-9) / 9;
    ans = ans * (l-10) / 10;
    ans = ans * (l-11) / 11;

    println!("{}", ans);
}
