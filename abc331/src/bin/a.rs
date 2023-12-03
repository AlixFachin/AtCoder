use proconio::input;

fn main() {
    input! {
       m_l: usize,
       d_l: usize,
       y: usize,
       m: usize,
       d: usize,
    };

    let mut y2: usize = y;
    let mut m2: usize = m;
    let mut d2: usize = d;

    if d < d_l {
        d2 = d2 + 1;
    } else {
        d2 = 1;
        if m < m_l {
            m2 = m + 1;
        } else {
            m2 = 1;
            y2 = y + 1;
        }
    }

    println!("{} {} {}", y2, m2, d2);

}
