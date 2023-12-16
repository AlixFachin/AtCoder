use proconio::input;

fn get_type(s: &str) -> u8 {

    return match s {
        "AB" | "BC" | "CD" | "DE" | "EA" | "BA" | "CB" | "DC" | "ED" | "AE" => 1,
        "AC" | "BD" | "CE" | "DA" | "EB" | "CA" | "DB" | "EC" | "AD" | "BE" => 2,
        _ => panic!("Pattern not recognized: {}", s),
    }

}

fn main() {
    input! {
       s: String,
       t: String,
    };

    if get_type(&s) == get_type(&t) {
        println!("Yes")
    } else {
        println!("No")
    }

}
