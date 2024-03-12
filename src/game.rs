use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
use std::{char, collections::btree_map::Range, fmt::Error};
pub use rand::{thread_rng, Rng, seq::SliceRandom};
//Own stuff lib.rs
use crate::{find_char_in_board, Board};

//////////////////////
//General game logic//
//////////////////////
pub fn game_over(input: char, player: Entity) -> bool {
    if input == 'q' {
        return true;
    }
    else if player.health == 0{
        return true;
    }
    return  false;
}
#[derive(Clone, Copy)]
struct BasicNeeds {
    starve: u32,
    hydrate: u32,
    convident: u32
}
#[derive(Clone, Copy)]
pub struct Entity {
    health: u32,
    strengh: u32,
    basic_needs: BasicNeeds
}
pub fn new_entity(_health: u32, _strengh: u32, _starve: u32, _hydrate: u32, _convident: u32) -> Entity {
    Entity {
        health: _health,
        strengh: _strengh,
        basic_needs: BasicNeeds {
            starve: _starve,
            hydrate: _hydrate,
            convident: _convident
        }
    }
}
pub fn show_entity_status(entity: Entity) {
    println!("<-=-=-=-=-=-=-=->");
    println!("<Health: {}", entity.health);
    println!("<Strengh: {}", entity.strengh);
    println!("<~~~~~~~~~~~~~~~>");
    println!("<Starvation {}", entity.basic_needs.starve);
    println!("<Hydration {}", entity.basic_needs.hydrate);
    println!("<Convinience {}", entity.basic_needs.convident);
    println!("<-=-=-=-=-=-=-=->");
}
//////////////////////////////
//General movement functions//
//////////////////////////////
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
////////////////
//User actions//
////////////////
pub fn handle_input(usr_input: char, board: &mut Board) {
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
pub fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
        }
    }   
}
////////////////////
//Preditor actions//
////////////////////
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
pub fn move_preditor(board: &mut Board) {
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