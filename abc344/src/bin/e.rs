use std::collections::HashMap;

use proconio::input;

const DEBUG: bool = false;
#[derive(Clone, Copy)]
struct Element {
    next: Option<usize>,
    prev: Option<usize>,
}

fn main() {
    input! {
       n: usize,
       a: [usize;n],
       q: usize,
    }

    let mut indexes: HashMap<usize, Element> = HashMap::new();

    if n == 1 {
        indexes.insert(0, Element { next: Some(a[0]), prev: None });
        indexes.insert(a[0], Element { next: None, prev: Some(0) });
    } else {
        indexes.insert(0, Element { next: Some(a[0]), prev: None });
        indexes.insert(a[0], Element { next: Some(a[1]), prev: Some(0) });

        for i in 1..n - 1 {
            indexes.insert(a[i], Element { next: Some(a[i + 1]), prev: Some(a[i - 1]) });
        }
        indexes.insert(a[n - 1], Element { next: None, prev: Some(a[n - 2]) });
    }

    for _query_index in 0..q {
        input! {
            t: char,
        }

        if t == '1' {
            // Insertion of y after x
            input! {
                x: usize,
                y: usize,
            }

            let element_x = indexes.get(&x).unwrap();
            if element_x.next.is_none() {
                indexes.insert(y, Element { next: None, prev: Some(x) });
                indexes.entry(x).and_modify(|e| {
                    e.next = Some(y);
                });
            } else {
                let element_next_index = &element_x.next.unwrap();
                indexes.insert(y, Element { next: element_x.next, prev: Some(x) });
                indexes.entry(x).and_modify(|e| {
                    e.next = Some(y);
                });
                indexes.entry(*element_next_index).and_modify(|f| {
                    f.prev = Some(y);
                });
            }
        } else {
            input! {
                x: usize,
            }

            // Deletion of x
            let &element_x = indexes.get(&x).unwrap();
            let element_after_x = element_x.next;
            if let Some(prev) = element_x.prev {
                indexes.entry(prev).and_modify(|e| e.next = Some(element_after_x.unwrap()));
            }
            if let Some(next) = element_x.next {
                indexes.entry(next).and_modify(|f| f.prev = element_x.prev);
            }
        }
    }
    let mut cur_next = indexes.get(&0).unwrap().next;
    while cur_next.is_some() {
        print!("{} ", cur_next.unwrap());
        cur_next = indexes.get(&cur_next.unwrap()).unwrap().next;
    }

    println!();
}
