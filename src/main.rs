use std::{fmt::Error, char};

pub use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
pub use colorized::*;

type Title = char;
type Board = Vec<Vec<Title>>;


fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
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

fn handle_input(usr_input: char, board: &mut Board) {
    let coordinates: Vec<(u32, u32)> = find_player_in_board(board);
    match usr_input {
        'w' => move_up(coordinates ,board),
        'a' => move_left(coordinates, board),
        's' => move_down(coordinates, board),
        'd' => move_right(coordinates, board),
        _ => println!("Error")
    }
}
fn init_board() -> Board {
    vec![vec!['#';16];16]
}
fn find_player_in_board(board: &mut Board) -> Vec<(u32, u32)> {
    let mut coordinates = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char == '@' {
                coordinates.push((x as u32, y as u32));
            }
        }
    }
    coordinates
}
fn set_player_in_board(board: &mut Board) {
    board[8][8] = '@';
}
fn game_over(input: char) -> bool {
    if input == 'q' {
        return true;
    }
    return  false;
}
fn print_board(board: &mut Board) {
    for (i, row) in board.iter().enumerate() {
        for (j,row) in row.iter().enumerate() {
            if row == &'@' {
                print!("{}",Colors::MagentaFg.value());
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
    print_board(&mut board);
    let mut usr_input:char = get_user_input();
    while  !game_over(usr_input) {
        print_board(&mut board);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board)
    }
    println!("Hello, world!");
}
