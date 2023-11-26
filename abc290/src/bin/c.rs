use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;n],
    }

    if DEBUG { println!("Before sort: {:?}",&a)}
    a.sort();
    if DEBUG { println!("after sort: {:?}",&a)}
    let filtered : Vec<usize> = a.iter().enumerate().filter(|(i,_x) | *i == n-1 || a[*i] != a[*i+1] ).map(|(_i,x)| *x).collect();
    if DEBUG { println!("after filtered: {:?}",&filtered)}
    
    for i in 0..filtered.len().min(k) {
        if filtered[i] != i {
            println!("{}",i);
            return;
        }
    }
    println!("{}",filtered.len().min(k));


}
