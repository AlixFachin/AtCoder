use proconio::input;

const DEBUG: bool = false;


fn solve(a: i64, m: i64, l: i64, r: i64) -> i64 {
    if l == r {
        if l % m == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let a2: i64;
    if l < a {
        a2 =  a + (((l - a) / m) * m) - 2 * m;
    } else {
        a2 = a;
    }

    let la = l - a2;
    let ra = r - a2;

    let kl = la / m;
    let kr = ra / m;

    if DEBUG {
        println!("(l/m)*m={}",(((l - a) / m) * m) );
        println!("New a is : {}", a2);
        println!("la: {},ra: {}, kl:  {}, kr:  {}, klmod:{}, krmod:{},  res: {}", la, ra, kl, kr, la % m, ra % m,  kr - kl);
    }
    
    let mut res = kr - kl;
    if la % m == 0 {
        res = res + 1;
    }

    return res;

}

#[test] 
fn test() {
    // both below
    assert_eq!(solve(5,3,-5, 0),2);    
    assert_eq!(solve(5,3,-4, 0),2);    
    assert_eq!(solve(5,3,-5, -1),2);
    assert_eq!(solve(5,3,-4, -1),2);
    // straddle
    assert_eq!(solve(5,3, -2, 7),3);
    assert_eq!(solve(5,3,-1, 7),3);
    assert_eq!(solve(5,3, -2, 8),4);
    assert_eq!(solve(5,3,-1, 8),4);
    // both above
    assert_eq!(solve(5,3, 7, 12),2);
    assert_eq!(solve(5,3,8, 12),2);
    assert_eq!(solve(5,3, 7, 11),2);
    assert_eq!(solve(5,3,8, 14),3);
}

fn main() {
    input! {
       a : i64,
       m: i64,
       l: i64,
       r: i64,
    };

    let res = solve(a, m, l, r);

    println!("{}", res);

}
