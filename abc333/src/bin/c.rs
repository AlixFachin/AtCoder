use proconio::input;
use itertools::iproduct ;

fn main() {
    input! {
       n: usize,
    }

    let number_list: Vec<usize> = vec![
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
        1111111111111,
        11111111111111,
    ];
    let mut sum_list: Vec<usize> = vec![];

    for (i, j, k) in iproduct!(number_list.iter(), number_list.iter(), number_list.iter()) {
        sum_list.push(*i + *j + *k);
    }
    sum_list.sort();
    sum_list.dedup();

    println!("{}", sum_list[n - 1]);
}
