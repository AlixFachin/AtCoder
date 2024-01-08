use proconio::input;

fn main() {
    input! {
       n: usize,
       s: [usize;n],
       t: [usize;n],
    };

    let mut min_received_time: Vec<Option<usize>> = vec![None];

    // Each series can potentially get to a min time somwhere
    for start_index in 0..n {
        let mut current_index = start_index;
        let mut current_time = t[start_index];
        for _i in 0..n {
            // test of minimum
            if let Some(min_time) = min_received_time[current_index] {
                if current_time < min_time {
                    min_received_time[current_index] = Some(current_time);
                }
            } else {
                min_received_time[current_index] = Some(current_time);
            }
            // go to next time
            current_time = current_time + s[current_index];
            current_index = (current_index + 1) % n;

        }
    }

    for i in 0..n {
        if let Some(min_time) = min_received_time[i] {
            println!("{}", min_time);
        } else {
            panic!("There is not supposed to be empty times remaining!")
        }
    }

}
