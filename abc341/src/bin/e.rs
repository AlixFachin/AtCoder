use ac_library::{Max,Segtree};
use proconio::{input, marker::Chars};


fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(char, usize,usize); q],
    }

    let mut st = Segtree::<Max<u8>>::new(n+1);

    for i in 1..n {
        if s[i-1] == s[i] {
            st.set(i, 1);
        }
    }

    for (t, l, r) in queries {
        if t == '1' {
            st.set(l-1, st.prod((l-1)..l)^1);
            st.set(r, st.prod(r..(r+1))^1);
        } else {
            println!("{}", if st.prod(l..r) == 0 { "Yes" } else { "No" });
        }
    }

}
