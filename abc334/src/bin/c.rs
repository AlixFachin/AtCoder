use proconio::input;

fn main() {
    input! {
       _n: usize,
       k: usize,
       a: [usize;k],
    }

    if k == 1 {
        println!("0");
        return;
    }
    

    let mut pairs = a.clone();
    pairs.sort();

    if pairs.len() % 2 == 0 {
        let mut distance = 0;
        let mut i = 0;
        while i < pairs.len() {
            distance = distance + (pairs[i + 1] - pairs[i]);
            i = i + 2;
        }
        println!("{}", distance);
    } else {
        // If the array is:
        // A B C D E F G
        // We will pre-compute with a first grouping
        // [ A B ] [ C D ] [ E F ] G
        // And we will pre-compute with a second grouping
        // A [ B C ] [ D E ] [ F G ]
        let mut first_group: Vec<usize> = vec![0;k];
        let mut second_group: Vec<usize> = vec![0;k];
        for i in 0..pairs.len() - 1 {
            if i % 2 == 1 {
                first_group[i] = first_group[i - 1];
            } else {
                if i == 0 {
                    first_group[i] = pairs[i + 1] - pairs[i];
                } else {
                    first_group[i] = first_group[i - 1] + (pairs[i + 1] - pairs[i]);
                }
            }
        }
        for i in (1..=pairs.len() - 1).rev() {
            if i % 2 == 0 {
                if i == pairs.len() - 1 {
                    second_group[i] = 0;
                } else {
                    second_group[i] = second_group[i + 1];
                }
            } else {
                second_group[i] = second_group[i + 1] + (pairs[i + 1] - pairs[i]);
            }
        }

        let mut min_dist: usize = usize::MAX;
        for i in 0..pairs.len() {
            // Now find a clever way to use first_group and second_group
            let dist;
            if i % 2 == 1 {
                dist = second_group[i] + first_group[i];
            } else {
                if i == 0 {
                    dist = second_group[1];
                } else if i == pairs.len() - 1 {
                    dist = first_group[i - 1];
                } else {
                    dist = first_group[i - 1] + second_group[i];
                }
            }
            min_dist = min_dist.min(dist);
        }
        println!("{}", min_dist);
    }
}
