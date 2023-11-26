use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let vc : Vec<char> = s.chars().collect();

    for i in 0..n {
        if i < n-1 {
            if (vc[i] == 'a' && vc[i+1] == 'b') || (vc[i] == 'b' && vc[i+1] == 'a') {
                println!("Yes");
                return;
            }
        } 
    }

    println!("No");    

}
