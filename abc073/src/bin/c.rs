use proconio::input;
const DEBUG: bool = false;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    }

    a.sort();
    let mut paper_stack: Vec<usize> = vec![];
    if DEBUG { println!("Sorted stuff: {:?}", a)}

    for x in a {
        if DEBUG { println!("Stack: {:?}, elem: {}", &paper_stack, &x);  }
        if let Some(v) = paper_stack.last() {            
            if DEBUG { println!("Comparing {} and {}", &x, &v)}
            if x == *v {                
                paper_stack.pop();
            } else {
                paper_stack.push(x);
            }
        } else {
            paper_stack.push(x);
        }
    }

    println!("{}", paper_stack.len());


}
