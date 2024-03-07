mod world;
use world::{init_board, Board};

mod entity;

use std::{char, collections::btree_map::Range, fmt::Error};
use rand::random;
pub use rand::{thread_rng, Rng, seq::SliceRandom};
pub use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
pub use colorized::*;



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

    let mut usr_input:char = 'x';
    while  !game_over(usr_input) {
        print_board(&mut board);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board);
        move_preditor(&mut board);
    }
    println!("Hello, world!");
}
