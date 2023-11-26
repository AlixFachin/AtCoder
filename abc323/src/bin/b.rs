use proconio::input;
use std::cmp::Ordering;

fn main() {

    input! {
        n: usize,
        games: [String; n],
    }

   let mut ranks = vec![0; n];

   for i in 0..n {
        ranks[i] = games[i].chars().fold(0,|acc,c| acc + if c == 'o' { 1} else { 0 } )
   }

   let mut v: Vec<(usize, &i32)> = ranks.iter().enumerate().collect();
   v.sort_by(|(a_idx,a_rank),(b_idx,b_rank)| if a_rank < b_rank { Ordering::Less } else { if a_rank > b_rank { Ordering::Greater} else { b_idx.cmp(a_idx)  }  } );

   let result = v.iter().rev().map(|(idx, rank)| (idx+1).to_string() ).reduce(|acc, x| acc + " " + &x  );

   println!("{}",result.unwrap());
}
