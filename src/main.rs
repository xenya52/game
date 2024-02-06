use std::{char, collections::btree_map::Range, fmt::Error};
use rand::random;
pub use rand::{thread_rng, Rng, seq::SliceRandom};
pub use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
pub use colorized::*;

type Title = char;
type Board = Vec<Vec<Title>>;

fn random_coordinates(board: &mut Board) -> Vec<usize> {
    let mut empty_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == '#' {
                empty_tiles.push((i, j));
            }
        }
    }

    if let Some(&(x, y)) = empty_tiles.choose(&mut thread_rng()) {
        return vec![x,y]
    }
    else {
        return vec![0,0]
    }
}

fn add_random_tile(board: &mut Board) {
    let mut empty_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == '#' {
                empty_tiles.push((i, j));
            }
        }
    }

    if let Some(&(x, y)) = empty_tiles.choose(&mut thread_rng()) {
        let value: char = if thread_rng().gen_bool(0.9) { 'x' } else { 'X' };
        board[x][y] = value;
    }
}

fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
        }
    }   
}
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

fn handle_input(usr_input: char, board: &mut Board) {
    let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
    if moveable(board, usr_input, &coordinates) {
        match usr_input {
            'w' => move_up(&coordinates ,board),
            'a' => move_left(&coordinates, board),
            's' => move_down(&coordinates, board),
            'd' => move_right(&coordinates, board),
            _ => println!("Error")
        }
    }
}
fn init_board() -> Board {
    let mut board = vec![vec!['#';16];16];
    let mut count = 0;
    loop {
        count += 1;
        add_random_tile(&mut board);
        if count == 10 {
            break;
        }
    }
    return board;
}
fn find_char_in_board(board: &mut Board, given: char) -> Vec<(u32, u32)> {
    let mut coordinates = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char == given {
                coordinates.push((x as u32, y as u32));
            }
        }
    }
    coordinates
}
fn set_player_in_board(board: &mut Board) {
    board[8][8] = '@';
}
fn set_preditor_in_board(board: &mut Board) {
    let c:Vec<usize> = random_coordinates(board);
    board[c[0]][c[1]] = 'ö';
}
fn move_preditor(board: &mut Board) {
    let possible_move: bool = rand::thread_rng().gen_bool(0.5);
    println!("{}",possible_move);
    if possible_move
     {
        let co_preditor: Vec<(u32,u32)> = find_char_in_board(board, 'ö');
        let co_player: Vec<(u32,u32)> = find_char_in_board(board, '@');
        // Assuming both vectors are guaranteed to have at least one element.
        // You should add checks to ensure they are not empty to avoid runtime panics.
        let resultx: i32 = co_preditor[0].0 as i32 - co_player[0].0 as i32; // Subtract x coordinates
        let resulty: i32 = co_preditor[0].1 as i32 - co_player[0].1 as i32; // Subtract y coordinates
        if resultx > 0 {
            if moveable(board, 'a', &co_preditor) {
                move_left(&co_preditor, board);
            }
            else{
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resultx < 0 {
            if moveable(board, 'd', &co_preditor) {
                move_right(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resulty < 0 {
            if moveable(board, 's', &co_preditor) {
                move_down(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resulty > 0 {
            if moveable(board, 'w', &co_preditor) {
                move_up(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
     }
}
fn game_over(input: char) -> bool {
    if input == 'q' {
        return true;
    }
    return  false;
}
fn try_move_alternative(board: &mut Board, co_preditor: &Vec<(u32, u32)>) {
    // Prioritätsliste der Richtungen, kann basierend auf dem Spielkontext angepasst werden
    let directions = vec!['w', 'a', 's', 'd']; // Oben, Links, Unten, Rechts als Beispiele

    for &direction in &directions {
        match direction {
            'w' if moveable(board, 'w', co_preditor) => {
                move_up(co_preditor, board);
                break; // Bewegung erfolgreich, Schleife verlassen
            },
            'a' if moveable(board, 'a', co_preditor) => {
                move_left(co_preditor, board);
                break; // Bewegung erfolgreich, Schleife verlassen
            },
            's' if moveable(board, 's', co_preditor) => {
                move_down(co_preditor, board);
                break; // Bewegung erfolgreich, Schleife verlassen
            },
            'd' if moveable(board, 'd', co_preditor) => {
                move_right(co_preditor, board);
                break; // Bewegung erfolgreich, Schleife verlassen
            },
            _ => continue, // Wenn Bewegung in dieser Richtung nicht möglich ist, nächste Richtung probieren
        }
    }
}
fn print_board(board: &mut Board) {
    for (i, row) in board.iter().enumerate() {
        for (j,row) in row.iter().enumerate() {
            if row == &'@' {
                print!("{}",Colors::BrightYellowFg.value());
            }
            else if row == &'ö' {
                print!("{}",Colors::RedFg.value())
            }
            else if row == &'x' || row == &'X' {
                print!("{}",Colors::BlackFg.value());
            }
            else {
                print!("{}",Colors::GreenFg.value());
            }
            print!("{}", row);
            print!("{}",Colors::Reset.value());
        }
        println!();
    }
}

fn main() {
    let mut board = init_board();
    set_player_in_board(&mut board);
    set_preditor_in_board(&mut board);
    let mut usr_input:char = 'x';
    while  !game_over(usr_input) {
        print_board(&mut board);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board);
        move_preditor(&mut board);
    }
    println!("Hello, world!");
}
