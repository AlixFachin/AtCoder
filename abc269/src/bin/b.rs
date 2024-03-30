use proconio::input;
use proconio::marker::Chars;

const DEBUG:bool = false;

fn main() {
    input! {
       s: [Chars; 10],
    };

    let mut a: Option<i32> = None;
    let mut b: Option<i32> = None;
    let mut c: Option<i32> = None;
    let mut d: Option<i32> = None;

    for i in 0..10 {
        if s[i].contains(&'#') {
            for j in 0..10 {
                if s[i][j] == '#' {
                    if c.is_none() {
                        c = Some((j+1) as i32);
                    }
                } else {
                    if c.is_some() && d.is_none() {
                        d = Some(j as i32);
                    }
                }
            }
            if d.is_none() {
                d = Some(10);
            }
            if a.is_none() {
                a = Some((i+1) as i32);
            }
        } else {
            if a.is_some() && b.is_none() {
                b = Some(i as i32);
            }
        }
    }
    if b.is_none() {
        b = Some(10);
    }
    println!("{} {}", a.unwrap(), b.unwrap());
    println!("{} {}", c.unwrap(), d.unwrap());

}
