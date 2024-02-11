use proconio::input;

const DEBUG: bool = false;

fn main() {
    input! {
       n: usize,
       m: usize,
       x: [usize; m],
    }

    // dist_if_broken[k,n] is the distance of the step k of the tour if the bridge n is broken;
    // let mut dist_if_broken = vec![0; n];
    // // For each bridge, we compute the tour length if this bridge is broken
    // for k in 0..(m-1) {
    //     let a = x[k].min(x[k + 1])-1;
    //     let b = x[k].max(x[k + 1])-1;

    //     for j in a..b {
    //         dist_if_broken[j] += a + n - b;
    //     }
    //     for j in b..(a+n) {
    //         dist_if_broken[j%n] += b - a;
    //     }
    // }
    // println!("{}", dist_if_broken.iter().min().unwrap());

    // delta_dist[k] is the difference between dist[k] and dist[(k+1)%n]
    let mut delta_dist: Vec<i64> = vec![0; n];
    let mut dist_0 = 0; // we compute the dist_0 to initiate the recursion
    for k in 0..(m-1) {
        let a = x[k].min(x[k + 1])-1;
        let b = x[k].max(x[k + 1])-1;

        if a == 0 {
            dist_0 += a + n - b;
        } else {
            dist_0 += b - a;
        }
        delta_dist[a] += n as i64 -2*((b-a)as i64);
        delta_dist[b] += 2*((b-a)as i64) - (n as i64);
    }

    if DEBUG {
        println!("dist_0 = {}", dist_0);
        println!("delta_dist = {:?}", delta_dist);
    }

    let mut min_dist: i64 = dist_0 as i64;
    let mut dist_i:i64 = dist_0 as i64;
    for i in 1..n {
        dist_i += delta_dist[i];
        min_dist = min_dist.min(dist_i);
    }

    println!("{}", min_dist);

}
