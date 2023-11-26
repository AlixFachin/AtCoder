use std::collections::BinaryHeap;
use std::cmp::Reverse;

use proconio::input;
use proconio::fastout;

const DEBUG: bool = false;

fn insert_val(top: &mut BinaryHeap<Reverse<usize>>, k: usize, val: usize) {

    if DEBUG {
        println!("Trying to insert {} into {:?}",val,top);
    }

    if top.len() == 0 {
        top.push(Reverse(val));
        return;
    }

    if top.len() == k as usize {
       let Reverse(first_element) = top.peek().unwrap(); 
        if val <= *first_element {
            return;
        }
        top.pop();
    
    }

    top.push(Reverse(val));
}

#[fastout]
fn main() {
    
    input! {
        n: usize,
        k: usize,
        p: [usize;n],
    }

    let mut top_k: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    
    for i in 0..n {
        insert_val(&mut top_k, k, p[i as usize]);
        if i >= k-1 {
            let Reverse(first_element) = top_k.peek().unwrap() ;
            println!("{}",*first_element);
        }
    }



}
