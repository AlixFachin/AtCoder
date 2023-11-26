use proconio::input;

fn reduce_mult(a:usize, b: usize) -> usize {

    if a % b == 0 {
        return reduce_mult(a/b, b);
    }
    return a;
}

fn main() {
    
    input! {
        n: usize,
    }

    let reduce2 = reduce_mult(n,2);
    let reduce3 = reduce_mult(reduce2,3);

    if reduce3 == 1 {
        println!("Yes");
    } else {
        println!("No");
    }

}
