use proconio::input;
use std::collections::HashMap;

const DEBUG:bool = false;


fn abs_dist(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn get_maximum_sum(k: &Vec<usize>, target: usize, p: usize) -> usize{
    if p == 0 {
        return k[0];
    }
    // Otherwise we will make a recursion
    // trying to get the maximum sum close to target
    let p_elem = k[p];
    let max_with_p;
    if p_elem > target {
        max_with_p = 0;
    } else {
        max_with_p = get_maximum_sum(k, target - p_elem, p-1);
    }
    let max_without_p = get_maximum_sum(k, target, p-1);
    if abs_dist(max_with_p + p_elem, target) < abs_dist(max_without_p, target) {
        return max_with_p + p_elem;
    } else {
        return max_without_p;
    }
}


fn main() {
    input! {
       // Insert input description here
       n: usize,
       mut k: [usize; n],
    };

    let elem_sum: usize = k.iter().sum();
    let target: usize = (elem_sum) / 2;
    let mut memo: HashMap<usize, usize> = HashMap::new();

    // The goal is to find a subset of k which sum is closest to target.
    let closest_to_target = get_maximum_sum(&k, target, n-1); 
    let result = closest_to_target.max(elem_sum - closest_to_target);
    println!("{}", result);

}
