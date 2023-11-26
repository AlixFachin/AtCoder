use proconio::input;
use itertools::{ iproduct};

const DEBUG: bool = false;

fn is_valid_row(n: usize, v: &Vec<Vec<char>>,i: usize) -> bool {
    let mut ca = 0;
    let mut cb = 0;
    let mut cc = 0;

    for j in 0..n {
        if v[i][j] == 'A' {
            ca = ca + 1;
        }
        if v[i][j] == 'B' {
            cb = cb + 1;
        }
        if v[i][j] == 'C' {
            cc = cc + 1;
        }
        if ca > 1 || cb > 1 || cc > 1 {
            return false;
        }            
    }
    return ca == 1 && cb == 1 && cc == 1;
}

fn is_valid_col(n: usize, v: &Vec<Vec<char>>, j: usize) -> bool {
    let mut ca = 0;
    let mut cb = 0;
    let mut cc = 0;

    for i in 0..n {
        if v[i][j] == 'A' {
            ca = ca + 1;
        }
        if v[i][j] == 'B' {
            cb = cb + 1;
        }
        if v[i][j] == 'C' {
            cc = cc + 1;
        }
        if ca > 1 || cb > 1 || cc > 1 {
            return false;
        }
    }
    return ca == 1 && cb == 1 && cc == 1;
}

fn get_first_row(n: usize, v: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for j in 0..n {
        if v[0][j] != '.' {
            s.push(v[0][j]);
        } else if v[1][j] != '.' {
            s.push(v[1][j]);
        } else {
            s.push(v[2][j]);
        }
    }
    return s;
}

fn get_first_col(n: usize, v: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for i in 0..n {
        if v[i][0] != '.' {
            s.push(v[i][0]);
        } else if v[i][1] != '.' {
            s.push(v[i][1]);
        } else {
            s.push(v[i][2]);
        }
    }
    return s;
}

#[test]
fn test_validity(){

    let matrix = vec![vec!['A','B','C'],vec!['B','C','A'], vec!['C','A','B']];
    assert!(is_valid_row(3, &matrix, 0));
    assert!(is_valid_row(3, &matrix, 1));
    assert!(is_valid_row(3, &matrix, 2));
    assert!(is_valid_col(3, &matrix, 0));
    assert!(is_valid_col(3, &matrix, 1));
    assert!(is_valid_col(3, &matrix, 2));

    let matrix_wrong = vec![vec!['A','C','C'],vec!['B','B','A'], vec!['A','A','A']];
    assert!(!is_valid_row(3, &matrix_wrong, 0));
    assert!(!is_valid_row(3, &matrix_wrong, 1));
    assert!(!is_valid_row(3, &matrix_wrong, 2));
    
    let bigger_mat = vec![vec!['.','B','C','.'],vec!['B','.','A','C'], vec!['.','A','B','C'],vec!['C','A','B','.']];
    assert_eq!(get_first_row(4, &bigger_mat),String::from("BBCC"));
    assert_eq!(get_first_col(4, &bigger_mat),String::from("BBAC"));
    
    let solution = vec![vec!['A','C','.','.','B'], vec!['.','B','A', '.', 'C'], vec!['C','.','B','A','.'], vec!['B','A','.','C','.'], vec!['.','.','C','B','A']];
    let tgt_row = String::from("ABCBC");
    let tgt_col = String::from("ACAAB");
    assert!(is_valid(5,&solution, &tgt_col, &tgt_row));

}



fn is_valid(n: usize, v: &Vec<Vec<char>>, r: &String, c: &String) -> bool {
    
    if DEBUG {  print_grid(n, v); }

    // Check that for each row, we have one and only once a
    let mut correct = true;
    for x in 0..n {
        correct = correct && is_valid_row(n, v, x);
        if !correct {return false;}
        correct = correct && is_valid_col(n, v, x);
        if !correct {return false;}
    }

    let first_row = get_first_row(n, v);
    let first_col = get_first_col(n,v);

    if DEBUG {
        println!("{}",first_row);
        println!("{}", first_col);
        println!("Test FR:{} FC:{}",first_row.eq(r),first_col.eq(c));
    }



    return first_row.eq(r) && first_col.eq(c);
}

fn print_grid(n: usize, v: &Vec<Vec<char>>) {
    for i in 0..n {
        for j in 0..n {
            print!("{}",v[i][j]);
        }
        println!("");
    }
    if DEBUG {println!("--------");}
}

fn substitute(template_row: &&str, target_string: &String, index: usize) -> Vec<char> {
    let target_letter = target_string.chars().nth(index).unwrap();
    let other_letters: Vec<char> = "ABC".chars().filter(|letter| *letter != target_letter).collect();
    let mut result: Vec<char> = vec![];
    for t in template_row.chars() {
        if t == '1' {
            result.push(target_letter);
        } else if t == '2' {
            result.push(other_letters[0]);
        } else if t == '3' {
            result.push(other_letters[1]);
        } else {
            result.push('.');
        }
    }
    return result;
}

#[test]
fn test_substitute() {

    let template_row_vec = vec![ "123..","12.3.","1.23.",".123.","12..3","1.2.3",".12.3","1..23",".1.23","..123",
    "132..","13.2.","1.32.",".132.","13..2","1.3.2",".13.2","1..32",".1.32","..132"];

    let target_string = String::from("ABCAA");
    let expected_results_0 = vec!["ABC..","AB.C.","A.BC.",".ABC.","AB..C","A.B.C",".AB.C","A..BC",".A.BC","..ABC","ACB..","AC.B.","A.CB.",".ACB.","AC..B","A.C.B",".AC.B","A..CB",".A.CB","..ACB"];
    let expected_results_1 = vec!["BAC..","BA.C.","B.AC.",".BAC.","BA..C","B.A.C",".BA.C","B..AC",".B.AC","..BAC","BCA..","BC.A.","B.CA.",".BCA.","BC..A","B.C.A",".BC.A","B..CA",".B.CA","..BCA"];

    for (i, template) in template_row_vec.into_iter().enumerate() {

        assert_eq!(substitute(&template, &target_string, 0),expected_results_0[i].chars().collect::<Vec<char>>());
        assert_eq!(substitute(&template, &target_string, 1),expected_results_1[i].chars().collect::<Vec<char>>());

    }

}

fn write_row_into_matrix(n: usize, row_to_be_written: &Vec<char>, matrix: &mut Vec<Vec<char>>, row_index: usize) {

    for i in 0..n {
        matrix[row_index][i] = row_to_be_written[i];
    }

}

fn main() {
    
    input! {
        n: usize,
        r: String,
        c: String,
    }

    let mut result=vec![vec![' ';n];n];

    // Possible combinations for each case
    let combinations;
    if n == 3 {
        combinations = vec![ "123", "132"];
        for (row1, row2, row3) in iproduct!(&combinations,&combinations, &combinations) {
            let to_write_1 = substitute(row1, &r, 0);
            let to_write_2 = substitute(row2, &r, 1);
            let to_write_3 = substitute(row3, &r, 2);
            
            write_row_into_matrix(n, &to_write_1, &mut result, 0);
            write_row_into_matrix(n, &to_write_2, &mut result, 1);
            write_row_into_matrix(n, &to_write_3, &mut result, 2);

            if is_valid(n, &result, &c, &r) {
                println!("Yes");
                print_grid(n, &result);
                return;
            }
        }
    } else if n == 4 {
        combinations = vec![ "123.", "12.3","1.23",".123","132.","13.2","1.32",".132"];
        for (row1, row2, row3, row4) in iproduct!(&combinations,&combinations, &combinations, &combinations) {

            let to_write_1 = substitute(row1, &r, 0);
            let to_write_2 = substitute(row2, &r, 1);
            let to_write_3 = substitute(row3, &r, 2);
            let to_write_4 = substitute(row4, &r, 3);
            
            write_row_into_matrix(n, &to_write_1, &mut result, 0);
            write_row_into_matrix(n, &to_write_2, &mut result, 1);
            write_row_into_matrix(n, &to_write_3, &mut result, 2);
            write_row_into_matrix(n, &to_write_4, &mut result, 3);

            // Need to write into result the combination created
            if is_valid(n, &result, &c, &r) {
                println!("Yes");
                print_grid(n, &result);
                return;
            }
        }
    }else {
        combinations = vec![ "123..","12.3.","1.23.",".123.","12..3","1.2.3",".12.3","1..23",".1.23","..123",
        "132..","13.2.","1.32.",".132.","13..2","1.3.2",".13.2","1..32",".1.32","..132"];
        for (row1, row2, row3, row4, row5) in iproduct!(&combinations,&combinations, &combinations, &combinations, &combinations) {

            let to_write_1 = substitute(row1, &r, 0);
            let to_write_2 = substitute(row2, &r, 1);
            let to_write_3 = substitute(row3, &r, 2);
            let to_write_4 = substitute(row4, &r, 3);
            let to_write_5 = substitute(row5, &r, 4);
            
            write_row_into_matrix(n, &to_write_1, &mut result, 0);
            write_row_into_matrix(n, &to_write_2, &mut result, 1);
            write_row_into_matrix(n, &to_write_3, &mut result, 2);
            write_row_into_matrix(n, &to_write_4, &mut result, 3);
            write_row_into_matrix(n, &to_write_5, &mut result, 4);


            if is_valid(n, &result, &c, &r) {
                println!("Yes");
                print_grid(n, &result);
                return;
            }
        }


    }
    
    println!("No");
}
