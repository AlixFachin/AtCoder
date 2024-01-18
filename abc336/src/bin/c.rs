use proconio::input;
const DEBUG:bool = false;

fn main() {
    input! {
         n: usize
    };

    let mut digits_5: Vec<u8> = vec![];
    // The key is to substract 1 from n now!!!
    let mut res = n-1;
    while res != 0 {
        digits_5.push((res % 5) as u8 );
        res /= 5;
    }

    if DEBUG {
        println!("digits_5: {:?}", digits_5);
    }

    // Now we need to rebuild the number
    let mut ans: usize = 0;
    for i in 0..digits_5.len() {
        if DEBUG {
            println!("i: {i}, ans= {ans}, to be added: {} ", 2*(digits_5[i] as usize)*((10 as usize).pow(i as u32)));
        }
        ans += 2*(digits_5[i] as usize)*((10 as usize).pow(i as u32));
    }
    println!("{}", ans );

}
