use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn main() {
    input! {
       mut s: Chars,
       q: usize,
    };

    let mut is_reversed = false;
    // Because insertion at the front is slow, 
    // we will keep the chars supposed to be inserted at the front in a separate stack.
    // At the end of the loop, we reverse this stack, then concatenate everything - and voila.
    let mut push_front_stack: Vec<char> = vec![];
    for _i in 0..q {
        input! {
            t: char,
        }
        if t  == '1' {
            is_reversed = !is_reversed;
            if DEBUG {
                println!("is_reversed: {:?}", is_reversed);
            }
        } else {
            input! {
                f: char,
                c: char,
            }

            if f == '1' && !is_reversed || f == '2' && is_reversed {
                // Regular case and f == '1' => insert at the beginning
                push_front_stack.push(c);
                if DEBUG {
                    println!("s (inserted {c} at beginning): {:?}", s);
                }
            } else {
                s.push(c);
                if DEBUG {
                    println!("s (inserted {c} at end): {:?}", s);
                }
            }
        }
    }

    push_front_stack.reverse();
    push_front_stack.append(s.as_mut());

    if is_reversed {
        push_front_stack.reverse();
    }

    println!("{}", push_front_stack.iter().collect::<String>());

}
