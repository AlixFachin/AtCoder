
use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;
const MAX_COST: usize = 100_000;


fn is_substring_matching(target: &Vec<char>, string_to_add: &String, start: usize) -> bool {
    if start + string_to_add.len() > target.len() {
        return false;
    }
    return string_to_add.chars().enumerate().fold(true, | acc, (i,c)| acc && (c == target[start+i]));
}

#[test]
fn test_substring_matching() {
    let target = vec!['a','b','c','d'];
    let string_to_add = "bc".to_string();
    assert_eq!(is_substring_matching(&target, &string_to_add, 1), true);
    assert_eq!(is_substring_matching(&target, &string_to_add, 0), false);
    assert_eq!(is_substring_matching(&target, &string_to_add, 2), false);
    assert_eq!(is_substring_matching(&target, &string_to_add, 3), false);
}

fn main() {
    input! {
       t: Chars,
       n: usize,
    };

    let mut pieces: Vec<Vec<String>> = vec![vec![];n];
    for i in 0..n {
        input! {
            a: usize,
        }
        for _j in 0..a {
            input! {
                s: String,
            }
            pieces[i].push(s);
        }
    }

    if DEBUG {
        println!("pieces: {:?}", pieces);
    }

    // Now we will prepare the DP table
    // The DP will contain the min cost that we can get for the first i characters (if any)
    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None;t.len()+1];n+1];
    // TO DO: Initialise the start layer
    for i in 0..n{
        // For each layer we have to add to the solutions the possibility to start from scratch with each pieces
        for k in 0..pieces[i].len() {
            let string_to_add = &pieces[i][k];
            if is_substring_matching(&t, string_to_add, 0) {
                // We start from scratch, the cost is 1, so it's the best solution for now
                dp[i][string_to_add.len()] = Some(1);
            }
        }
        // If we already had solutions, we have to check the combinations with the previous layer
        if i > 0 {
            // For each layer, we look at the solutions we had in the previous layer
            for j in 0..t.len()+1 {
                if let Some(cost) = dp[i-1][j] {
                    // Each time we had a solution in the previous layer, we can do nothing and propagate this solution to the next layer``
                    dp[i][j] = Some(cost.min(dp[i][j].unwrap_or(MAX_COST)));
                    // And we have to see if we can combine this previous solution with the string
                    for k in 0..pieces[i].len() {
                        let string_to_add = &pieces[i][k];
                        // Now we need to see if the string to add "fits" with the target string t, at the right position
                        if is_substring_matching(&t, string_to_add, j) {
                            // If it fits, we have to see if we already have a way to get to this length. If we do, we have to take the minimum
                            dp[i][j+string_to_add.len()] = Some(dp[i][j + string_to_add.len()].unwrap_or(MAX_COST).min(cost + 1));
                        }
                    }
                }
            }
        }

        if DEBUG {
            println!("dp[{i}]: {:?}", dp[i]);
        }

    }
    if let Some(cost) = dp[n-1][t.len()] {
        println!("{}", cost);
    } else {
        println!("-1");
    }

}
