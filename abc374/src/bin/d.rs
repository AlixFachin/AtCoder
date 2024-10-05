use proconio::input;

const DEBUG:bool = false;

#[test]
fn test_dist() {
    assert_eq!(get_dist((0,0), (0,0)), 0.0);
    assert_eq!(get_dist((0,0), (1,0)), 1.0);
    assert_eq!(get_dist((0,0), (0,1)), 1.0);
    assert_eq!(get_dist((0,0), (1,1)), 2.0_f64.sqrt());
    assert_eq!(get_dist((0,0), (2,2)), 8.0_f64.sqrt());
}

fn get_dist(a: (i32,i32), b: (i32,i32)) -> f64 {
    let x = (a.0 - b.0) as f64;
    let y = (a.1 - b.1) as f64;
    return (x*x + y*y).sqrt();
}


// Compute the time required to print all the segments in the queue passed in parameter
fn compute_time(segments: &Vec<(i32,i32,i32,i32)>, from: (i32,i32), s: f64, t: f64) -> f64 {
    if segments.len() == 1 {
        // Then we move compute the time necessary to move
        let segment_length = get_dist((segments[0].0, segments[0].1), (segments[0].2, segments[0].3));
        let start_from_a = get_dist(from, (segments[0].0, segments[0].1))/s + segment_length/t;
        let start_from_b = get_dist(from, (segments[0].2, segments[0].3))/s + segment_length/t;
        return start_from_a.min(start_from_b);
    }
    // Otherwise we will look at all combinations starting from each segment of the segment list
    let mut min_dist = f64::MAX;
    for i in 0..segments.len() {
        let mut queue = segments.clone();
        queue.remove(i);
        let (xa,ya,xb,yb) = segments[i];
        let segment_length = get_dist((xa, ya), (xb, yb));
        // First, starting from A
        let go_to_a_and_trace_to_b = get_dist(from, (xa, ya))/s + segment_length/t;
        let remainder_from_b = compute_time(&queue,(xb, yb), s, t );
        // Then, starting from B
        let go_to_b_and_trace_to_a = get_dist(from, (xb, yb))/s + segment_length/t;
        let remainder_from_a = compute_time(&queue,(xa, ya), s, t );
        min_dist = min_dist.min(go_to_a_and_trace_to_b+remainder_from_b);
        min_dist = min_dist.min(go_to_b_and_trace_to_a+remainder_from_a);
    }
    return min_dist;

}


fn main() {
    input! {
      n: usize,
      s: usize,
      t: usize,
      segments: [(i32, i32, i32, i32); n]
    };

    // The goal is to find the quickest time to print all the segments
    // The brute force is:
    // For each segment, compute the time to take to print all the other segments 
    // starting from the point B
    // compute the time to print all other segments starting from point A
    // Add the time to print the segment itself
    let result = compute_time(&segments, (0,0), s as f64, t as f64);

    println!("{}", result);

}
