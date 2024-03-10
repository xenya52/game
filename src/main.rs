use std::{char, collections::btree_map::Range, fmt::Error};
use rand::random;
pub use rand::{thread_rng, Rng, seq::SliceRandom};
pub use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
pub use colorized::*;

pub type Board = Vec<Vec<Title>>;
type Title = char;

fn get_rdm_xy(board: &mut Board) -> Vec<usize> {
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
    let value: char = if thread_rng().gen_bool(0.9) { 'x' } else { 'X' };
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = value;
}
fn set_player_in_board(board: &mut Board) {
    board[8][8] = '@';
}
fn set_preditor_in_board(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = 'ö';
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
    set_player_in_board(&mut board);
    set_preditor_in_board(&mut board);
    return board;
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
fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
        }
    }   
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
fn handle_input(usr_input: char, board: &mut Board) {
    let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
    if alternative_preditor_move(board, usr_input, &coordinates) {
        match usr_input {
            'w' => move_up(coordinates ,board),
            'a' => move_left(coordinates, board),
            's' => move_down(coordinates, board),
            'd' => move_right(coordinates, board),
            _ => println!("Error")
        }
    }
}
fn move_up(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
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
fn move_down(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
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

fn move_right(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
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

fn move_left(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
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
fn alternative_preditor_move(board: &mut Board, usr_input:char, coordinates: &Vec<(u32, u32)>) -> bool {
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
            if alternative_preditor_move(board, 'a', &co_preditor) {
                move_left(co_preditor, board);
            }
        }
        else if resultx < 0 {
            if alternative_preditor_move(board, 'd', &co_preditor) {
                move_right(co_preditor, board);
            }
        }
        else if resulty < 0 {
            if alternative_preditor_move(board, 's', &co_preditor) {
                move_down(co_preditor, board);
            }
        }
        else if resulty > 0 {
            if alternative_preditor_move(board, 'w', &co_preditor) {
                move_up(co_preditor, board);
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
fn main() {
    let mut board = init_board();

    let mut usr_input:char = 'x';
    while  !game_over(usr_input) {
        print_board(&mut board);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board);
        move_preditor(&mut board);
    }
    println!("Hello, world!");
}
