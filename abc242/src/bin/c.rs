use proconio::input;

const MODULO: u64 = 998244353;



fn main() {

    input! {
        n: usize,
    }

    let mut dp : Vec<[u64;10]> = vec![];

    if n == 1 {
        println!("{}",9);
        return;
    }

    dp.push([1;10]);
    for i in 1..n {
        let step_before = dp[i-1];
        let mut step: [u64;10] = [0;10];
        step[1] = (step_before[1] + step_before[2])% MODULO;
        step[9] = (step_before[9] + step_before[8])% MODULO;
        for j in 2..=8 {
            step[j] = (step_before[j-1] + step_before[j] + step_before[j+1]) % MODULO;
        }

        dp.push(step);
    }

    let mut result: u64 = 0;
    for i in 1..=9 {
        result = (result + dp[n-1][i]) % MODULO;
    }

    println!("{}",result);

}
