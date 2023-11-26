use std::fmt::format;

use proconio::input;

fn main() {
   input! {
    s: String,
   };

  let res = s.chars().filter(|c| *c != 'a' && *c != 'i' && *c != 'o' && *c != 'u' && *c != 'e').fold("".to_string(),|prev, c| format!("{}{}",prev,c)  );
  println!("{}",res); 

}
