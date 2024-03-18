use std::{borrow::Borrow, vec};

pub use rand::{thread_rng, Rng, seq::SliceRandom};
pub use colorized::*;
//Own stuff lib.rs
use crate::{get_rdm_xy, is_inside_the_grid};

/////////
//Board//
/////////
pub type Board = Vec<Vec<char>>;

////////////////////
//World generation//
////////////////////
fn add_random_mountain(board: &mut Board) -> bool {
    let c = get_rdm_xy(board);
    let c_space = vec![c[0] + 2, c[1] + 2];

    if is_inside_the_grid(board, &c) 
    && is_inside_the_grid(board, &c_space) {
        for i in 0..3 { // Iteriert über Zeilen
            for j in 0..3 { // Iteriert über Spalten
                let value = if thread_rng().gen_bool(0.5) { 'x' } else { 'X' };
                board[c[0] + i][c[1] + j] = value;
            }
        }
        true
    } else {
        false
    }
}
fn add_radom_water(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    let c_space: Vec<usize> = vec![c[0] + 1, c[1] + 1];
    if is_inside_the_grid(board, &c) 
    && is_inside_the_grid(board, &c_space) {
        for i in 0..2 {
            for j in 0..2 {
                let value = '~';
                board[c[0] + i][c[1] + j] = value;
            }
        }
    }
}
fn add_radom_food(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    let c_space:Vec<usize> = vec![c[0] + 3, c[1] + 3];
    let food: char = '+';
    let wood: char = '|';
    if is_inside_the_grid(board, &c)
    && is_inside_the_grid(board, &c_space) {
            board[c[0] + 2][c[1]] = food;
            board[c[0] + 2][c[1] + 1] = food;
            board[c[0] + 2][c[1] + 2] = food;
            board[c[0] + 2][c[1] + 3] = food;
            board[c[0] + 1][c[1] + 1] = food;
            board[c[0] + 1][c[1] + 2] = food;
            board[c[0] + 3][c[1] + 1] = wood;
            board[c[0] + 3][c[1] + 2] = wood;
        }
    }
fn set_player_in_board(board: &mut Board) {
    board[8][8] = '@';
}
fn set_preditor_in_board(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = 'ö';
}
pub fn init_board() -> Board {
    let mut board = vec![vec!['#';32];16];
    let mut count = 0;
    //Generate barriers
    print!("Generate mountians ... ");
    loop {
        count += 1;
        add_random_mountain(&mut board);
        if count == 10 {
            break;
        }
    }
    count = 0;
    println!("Done!");
    //Generate water
    print!("Generate water ... ");
    loop {
        count += 1;
        add_radom_water(&mut board);
        if count == 3 {
            break;
        }
    }
    count = 0;
    println!("Done!");
    //Generate food
    print!("Generate food ... ");
    loop {
        count += 1;
        add_radom_food(&mut board);
        if count == 4 {
            break;
        }
    }
    println!("Done!");
    //Set preditor in board
    print!("Set preditor in board ... ");
    set_preditor_in_board(&mut board);
    println!("Done!");
    //Set player in board
    print!("Set player in board ... ");
    set_player_in_board(&mut board);
    println!("Done!");
    return board;
}
////////////////
//Show a board//
////////////////
pub fn print_board(board: &mut Board) {
    for (i, row) in board.iter().enumerate() {
        for (j,row) in row.iter().enumerate() {
            if row == &'@' {
                print!("{}",Colors::BrightYellowFg.value());
            }
            else if row == &'ö' {
                print!("{}",Colors::WhiteFg.value())
            }
            else if row == &'x' || row == &'X' {
                print!("{}",Colors::BrightBlackFg.value());
            }
            else if row == &'~' {
                print!("{}", Colors::BrightBlueFg.value());
            }
            else if row == &'+' || row == &'|' {
                print!("{}", Colors::RedFg.value());
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