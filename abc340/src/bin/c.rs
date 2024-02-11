use std::collections::HashMap;
use proconio::input;

fn f(cache: &mut HashMap<usize,usize>, n: usize) -> usize {

    if n == 1 {
        return 0;
    }

    if let Some(&v) = cache.get(&n) {
        return v;
    }

    let a1 = f(cache, n/2);
    let a2 = f(cache, (n+1)/2);
    cache.insert(n, a1 + a2 + n);
    return a1 + a2 + n;
}


fn main() {
    input! {
       n: usize,
    };

    let mut cache: HashMap<usize,usize> = HashMap::new();

    println!("{}", f(&mut cache, n));

}
