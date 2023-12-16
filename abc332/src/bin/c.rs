use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
       n: i32,
       m: i32,
       s: Chars,
    };

    let mut stock_plain = m;
    let mut stock_logo = 0;
    let mut min_logo = 0;
    for c in s {
        match &c {
            '0' => {
                // Washing all shirts
                stock_plain = m;
                stock_logo = 0;
            },
            '1' => {
                // Go out on a meal, use a plain T-shirt if possible or a logo otherwise
                if stock_plain > 0 {
                    stock_plain = stock_plain - 1;
                } else {
                    stock_logo = stock_logo - 1;
                    min_logo = min_logo.min(stock_logo);
                }
            },
            '2' => {
                stock_logo = stock_logo - 1;
                min_logo = min_logo.min(stock_logo);
            },
            _ => panic!("Unknown activity code!"),
        }
    }

    println!("{}",-min_logo);
    
}
