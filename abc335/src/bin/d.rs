use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Tak,
    Dragon(usize),
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}


fn main() {
    input! {
       n: usize,
    };

    let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Empty; n]; n];
    grid[(n-1)/2][(n-1)/2] = Tile::Tak;

    let mut head_coord = (0,0);
    grid[head_coord.0][head_coord.1] = Tile::Dragon(1);
    let mut current_index:usize = 2;
    let mut current_direction = Direction::Right;
    while current_index < n * n {
        // Trying to determine the next coordinate
        // Can we go right?
        if current_direction == Direction::Right && head_coord.1 + 1 < n && grid[head_coord.0][head_coord.1 + 1] == Tile::Empty {
            head_coord.1 += 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            continue;
        }
        if current_direction == Direction::Down && head_coord.0 + 1 < n && grid[head_coord.0 + 1][head_coord.1] == Tile::Empty {
            head_coord.0 += 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            continue;
        }
        if current_direction == Direction::Left && head_coord.1 > 0 && grid[head_coord.0][head_coord.1 - 1] == Tile::Empty {
            head_coord.1 -= 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            continue;
        }
        if current_direction == Direction::Up && head_coord.0 > 0 && grid[head_coord.0 - 1][head_coord.1] == Tile::Empty {
            head_coord.0 -= 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            continue;
        }
        // Now we have to change direction - let's see where we can go
        if head_coord.1 + 1 < n && grid[head_coord.0][head_coord.1 + 1] == Tile::Empty {
            head_coord.1 += 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            current_direction = Direction::Right;
            continue;
        }
        // Can we go down?
        if head_coord.0 + 1 < n && grid[head_coord.0 + 1][head_coord.1] == Tile::Empty {
            head_coord.0 += 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            current_direction = Direction::Down;
            continue;
        }
        // Can we go left?
        if head_coord.1 > 0 && grid[head_coord.0][head_coord.1 - 1] == Tile::Empty {
            head_coord.1 -= 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            current_direction = Direction::Left;
            continue;
        }
        // Can we go up?
        if head_coord.0 > 0 && grid[head_coord.0 - 1][head_coord.1] == Tile::Empty {
            head_coord.0 -= 1;
            grid[head_coord.0][head_coord.1] = Tile::Dragon(current_index);
            current_index += 1;
            current_direction = Direction::Up;
            continue;
        }
    }

    for row in grid {
        for tile in row {
            match tile {
                Tile::Empty => print!("  "),
                Tile::Tak => print!("T "),
                Tile::Dragon(i) => print!("{} ", i),
            }
        }
        println!();
    }    

}
