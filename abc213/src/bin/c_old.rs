use proconio::input;

const DEBUG: bool = true;

#[derive(Clone, Debug)]
enum Card {
    Empty, // no card (after deletion)
    Number(usize),
    Blank,
}

fn set_location_to(table: &mut Vec<Vec<Card>>, i: usize, j: usize, new_val: usize) {
    table[i][j] = Card::Number(new_val);
}

/**
 * A row can be deleted if it has only some Blank cards and some Empty cards.
 * If it has only empty slots, it has already been deleted - so cannot be deleted again
 */
fn is_row_deletable(table: &Vec<Vec<Card>>, i: usize) -> bool {
    let mut count_blanks = 0;
    for j in 0..table[i].len() {
        match table[i][j] {
            Card::Blank => {
                count_blanks = count_blanks + 1;
            }
            Card::Number(_) => {
                return false;
            }
            Card::Empty => (),
        }
    }
    // when we reach here it means there are no numbers
    return count_blanks != 0;
}

fn is_col_deletable(table: &Vec<Vec<Card>>, j: usize) -> bool {
    let mut count_blanks = 0;
    for i in 0..table.len() {
        match table[i][j] {
            Card::Blank => {
                count_blanks = count_blanks + 1;
            }
            Card::Number(_) => {
                return false;
            }
            Card::Empty => (),
        }
    }
    // when we reach here it means there are no numbers
    return count_blanks != 0;
}

#[test]
fn test_is_deletable() {
    let table = vec![
        vec![Card::Blank, Card::Blank, Card::Empty],
        vec![Card::Empty, Card::Blank, Card::Empty],
        vec![Card::Empty, Card::Empty, Card::Empty],
        vec![Card::Number(1), Card::Blank, Card:: Empty]
    ];

    assert!(is_row_deletable(&table, 0));
    assert!(is_row_deletable(&table, 1));
    assert!(!is_row_deletable(&table, 2));
    assert!(!is_row_deletable(&table, 3));

    assert!(!is_col_deletable(&table, 0));
    assert!(is_col_deletable(&table, 1));
    assert!(!is_col_deletable(&table, 2));


}

fn delete_row(table: &mut Vec<Vec<Card>>, i: usize) {
    for j in 0..table[i].len() {
        table[i][j] = Card::Empty;
    }
}

fn delete_col(table: &mut Vec<Vec<Card>>, j: usize) {
    for i in 0..table.len() {
        table[i][j] = Card::Empty;
    }
}

fn print_coordinates(table: &Vec<Vec<Card>>, i0: usize, j0: usize) {
    // getting the column
    let mut count_empty = 0;
    for j in 0..j0 {
        match table[i0][j] {
            Card::Empty => {
                count_empty = count_empty + 1;
            }
            _ => (),
        }
    }
    let final_j = j0 - count_empty;
    count_empty = 0;
    for i in 0..i0 {
        match table[i][j0] {
            Card::Empty => {
                count_empty = count_empty + 1;
            }
            _ => (),
        }
    }
    let final_i = i0 - count_empty;
    println!("{} {}", final_i + 1, final_j + 1);
}

fn display_table(table: &Vec<Vec<Card>>) {
    println!("-=-=-=-=-=-=-=-=-=-=-=");
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            match table[i][j] {
                Card::Empty => print!("_"),
                Card::Blank => print!("*"),
                Card::Number(x) => print!("{}", x),
            }
        }
        println!("");
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        cards: [(usize,usize);n],
    }

    let mut table: Vec<Vec<Card>> = vec![vec![Card::Blank;w];n];

    for i in 0..n {
        set_location_to(&mut table, cards[i].0 - 1, cards[i].1 - 1, i + 1);
    }

    if DEBUG {
        display_table(&table);
    }

    let mut finished = false;
    while !finished {
        let mut has_deleted_something = false;
        for i in 0..h {
            if is_row_deletable(&table, i) {
                if DEBUG {
                    println!("Row {} is deletable!", i);
                }
                delete_row(&mut table, i);
                has_deleted_something = true;
                continue;
            }
        }
        for j in 0..w {
            if is_col_deletable(&table, j) {
                if DEBUG {
                    println!("Col {} is deletable!", j);
                }
                delete_col(&mut table, j);
                has_deleted_something = true;
                continue;
            }
        }
        if DEBUG {
            display_table(&table);
        }
        if !has_deleted_something {
            finished = true;
        }
    }

    // Then we need to display the coordinates
    for i in 0..n {
        print_coordinates(&table, cards[i].0 - 1, cards[i].1 - 1);
    }
}
