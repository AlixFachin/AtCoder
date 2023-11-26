use proconio::input;
fn main() {
    
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    // Can only be starting at points anyway

    let mut max_presents = 0;
    
    let mut a_sorted = a.clone();
    a_sorted.sort();
    
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;
    let mut cur_presents: usize = 0;

    // println!("{:?}",a_sorted);
    while end_index < n {
        // println!("Start loop: BI={} EI={}",start_index,end_index);        
        while end_index < n && a_sorted[end_index] < a_sorted[start_index] + m {
            // println!("   small loop - ei={}, ai={}, cp={}",end_index, a_sorted[end_index],cur_presents);
            end_index = end_index + 1;
            cur_presents = cur_presents + 1;
        }
        if cur_presents > max_presents {
            max_presents = cur_presents;
        }
        // Skipping to the next one, we decrease cur_presents because Ai goes away
        cur_presents = cur_presents - 1;
        start_index = start_index + 1;
    }

    println!("{}",max_presents);

}
