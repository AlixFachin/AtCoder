use proconio::input;
use proconio::marker::Chars;

const DEBUG: bool = false;

fn get_frequency_string(s: &str) -> String {
    let mut vec_code: Vec<usize> = vec![0; 26];
    for c in s.chars() {
        vec_code[c as usize - 'a' as usize] += 1;
    }
    let mut result = String::new();
    for i in 0..26 {
        result = format!("{},{}", &result, vec_code[i].to_string());
    }
    
    return result;
}

fn main() {
    input! {
       n: usize,
       sources: [Chars ; n],
    };

    // First, we parse all the strings
    let mut parsed_sources: Vec<String> = Vec::new();
    for source in sources {
        parsed_sources.push(get_frequency_string(&source.iter().collect::<String>()));
    }
    parsed_sources.sort();

    if DEBUG {
        println!("{:?}", parsed_sources);
    }

    let mut i_ref: usize = 0;
    let mut count_anagrams = 0;
    for i in 0..n {
        if parsed_sources[i] == parsed_sources[i_ref] {
            continue;
        } else {
            if (i - i_ref) > 1 {
                count_anagrams += (i - i_ref - 1) * (i - i_ref) / 2;
                if DEBUG {
                    println!("{} {} - gives {}", i_ref, i, (i - i_ref - 1) * (i - i_ref) / 2);
                }
            }
            i_ref = i;
        }
    }
    if (n - i_ref) > 1 {
        count_anagrams += (n - i_ref - 1) * (n - i_ref) / 2;
        if DEBUG {
            println!("{} {} - gives {}", i_ref, n, (n - i_ref - 1) * (n - i_ref) / 2);
        }
    }


    println!("{}", count_anagrams);

}
