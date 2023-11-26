use proconio::input;

fn main() {

    input! {
        s: String,
    }

    let v = s.chars().enumerate().filter(| (i,_) | *i > 0 && *i % 2 == 1 )
        .fold(true,|acc,x| acc && (x.1 == '0'));
    
    if v {
        println!("Yes");
    } else {
        println!("No");
    }

}
