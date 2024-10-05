use proconio::input;

const DEBUG:bool = false;

fn main() {
    input! {
       s: String
    };

    let n = s.len();
    let last_three = s[(n-3)..].to_string();
    if last_three == "san" {
        println!("Yes");
    } else {
        println!("No");
    }

}
