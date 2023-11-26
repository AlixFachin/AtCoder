// Simple hash with parens
use std::collections::HashMap;
use proconio::input;

/**
 * tourist 3858
ksun48 3679
Benq 3658
Um_nik 3648
apiad 3638
Stonefeang 3630
ecnerwala 3613
mnbvmar 3555
newbiedmy 3516
semiexp 3481
 */
fn main() {

    let mut scores = HashMap::new();

    scores.insert("tourist", 3858 );
    scores.insert("ksun48", 3679 );
    scores.insert("Benq", 3658 );
    scores.insert("Um_nik", 3648 );
    scores.insert("apiad", 3638 );
    scores.insert("Stonefeang", 3630 );
    scores.insert("ecnerwala", 3613 );
    scores.insert("mnbvmar", 3555 );
    scores.insert("newbiedmy", 3516 );
    scores.insert("semiexp", 3481 );
   
    input! {
        s: String,
    }

    println!("{:?}",scores.get(&s.as_str()).unwrap());

}
