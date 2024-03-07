

fn move_up(coordinates: &Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        // Make sure we are not on the first row and the coordinates are within the board boundaries
        if *y > 0 && *y as usize <= board.len() && *x as usize <= board[0].len() {
            let x_usize = *x as usize;
            let y_usize = *y as usize;

            // Swap the character with the element above it
            let temp = board[y_usize - 1][x_usize];
            board[y_usize - 1][x_usize] = board[y_usize][x_usize];
            board[y_usize][x_usize] = temp;
        }
    }
}
fn move_down(coordinates: &Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        if *y as usize + 1 < board.len() {
            let x_usize = *x as usize;
            let y_usize = *y as usize;

            let temp = board[y_usize + 1][x_usize];
            board[y_usize + 1][x_usize] = board[y_usize][x_usize];
            board[y_usize][x_usize] = temp;
        }
    }
}

fn move_right(coordinates: &Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        if *x as usize + 1 < board[0].len() {
            let x_usize = *x as usize;
            let y_usize = *y as usize;

            let temp = board[y_usize][x_usize + 1];
            board[y_usize][x_usize + 1] = board[y_usize][x_usize];
            board[y_usize][x_usize] = temp;
        }
    }
}

fn move_left(coordinates: &Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        if *x > 0 {
            let x_usize = *x as usize;
            let y_usize = *y as usize;

            let temp = board[y_usize][x_usize - 1];
            board[y_usize][x_usize - 1] = board[y_usize][x_usize];
            board[y_usize][x_usize] = temp;
        }
    }
}
fn moveable(board: &mut Board, usr_input:char, coordinates: &Vec<(u32, u32)>) -> bool {
    if let Some((x, y)) = coordinates.get(0) {
        if *x > 0 {
            let mut x_usize = *x as usize;
            let mut y_usize = *y as usize;

            match usr_input {
                'w' => y_usize -= 1,
                'a' => x_usize -= 1,
                's' => y_usize += 1,
                'd' => x_usize += 1,
                _ => println!("Error")
            }

            if board[y_usize][x_usize] == '#' {
                return true;
            }
        }
    }
    return false;
}