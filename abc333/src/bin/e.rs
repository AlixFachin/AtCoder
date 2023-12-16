use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n: usize,
       events: [(usize,usize);n],
    };
    
    let mut potions_stack: Vec<Vec<usize>> = vec![vec![];n];

    let mut i = 0;
    for (t,x) in &events {
        if *t == 1 {
            // Stacking the potion
            potions_stack[x-1].push(i);
        } else {
            // Using the last potion which we took
            if potions_stack[x-1].len() == 0 {
                println!("-1");
                return;
            }
            potions_stack[x-1].pop();
        }
        i=i+1;
    }

    // Now all the potions remaining in the stacks will be useless, we can choose not to take them
    let mut potions_not_taken: HashSet<usize> = HashSet::new();
    for stack in potions_stack {
        for x in stack {
            potions_not_taken.insert(x);
        }
    }

    // Now we replay the adventure, counting how many potions we take
    let mut total_potions = 0;
    let mut max_potions = 0;
    let mut i = 0;
    let mut decisions_string: Vec<&str> = vec![];
    for (t,_x) in &events {
        if *t == 1 {
            if potions_not_taken.contains(&i) {
                decisions_string.push("0");
            } else {
                total_potions = total_potions + 1;
                max_potions = max_potions.max(total_potions);
                decisions_string.push("1");
            }
        } else {
            total_potions = total_potions - 1;
        }
        i = i +1;
    }
    println!("{}", max_potions);
    println!("{}", decisions_string.join(" "));

}
