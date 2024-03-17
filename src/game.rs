use crossterm_input::input;
use std::char;
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
    else if dead_entity(player) {
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
    actions: u64,
    basic_needs: BasicNeeds
}
pub fn new_entity(_health: u32, _strengh: u32, _actions: u64, _starve: u32, _hydrate: u32, _convident: u32) -> Entity {
    Entity {
        health: _health,
        strengh: _strengh,
        actions: _actions,
        basic_needs: BasicNeeds {
            starve: _starve,
            hydrate: _hydrate,
            convident: _convident
        }
    }
}
pub fn show_entity_status(entity: &Entity) {
    println!("<-=-=-=-=-=-=-=->");
    println!("<Health: {}", entity.health);
    println!("<Strengh: {}", entity.strengh);
    println!("<Total actions: {}", entity.actions);
    println!("<~~~~~~~~~~~~~~~>");
    println!("<Starvation {}", entity.basic_needs.starve);
    println!("<Hydration {}", entity.basic_needs.hydrate);
    println!("<Convinience {}", entity.basic_needs.convident);
    println!("<-=-=-=-=-=-=-=->");
}
pub fn entity_moved(entity: &mut Entity) {
    entity.actions += 1;
    entity.basic_needs.starve -= if thread_rng().gen_bool(0.9) { 0 } else { 1 };
    entity.basic_needs.hydrate -= if thread_rng().gen_bool(0.9) { 0 } else { 2 };
}
pub fn dead_entity(entity: Entity) -> bool 
{
    if entity.health <= 0 ||
       entity.basic_needs.hydrate <= 0 ||
       entity.basic_needs.starve <= 0 {
        true
    }
    else {
        false
    }
}
//////////////////////////////
//General movement functions//
//////////////////////////////
fn move_up(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        // Swap the character with the element above it
        let temp = board[y_usize - 1][x_usize];
        board[y_usize - 1][x_usize] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}
fn move_down(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize + 1][x_usize];
        board[y_usize + 1][x_usize] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}

fn move_right(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize][x_usize + 1];
        board[y_usize][x_usize + 1] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}

fn move_left(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = coordinates.get(0) {
        let x_usize = *x as usize;
        let y_usize = *y as usize;

        let temp = board[y_usize][x_usize - 1];
        board[y_usize][x_usize - 1] = board[y_usize][x_usize];
        board[y_usize][x_usize] = temp;
    }
}
fn move_possibilites(board: &mut Board, usr_input:char, coordinates: &Vec<(u32, u32)>, entity: &mut Entity) -> bool {
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
            if board[y_usize][x_usize] == '~' || board[y_usize][x_usize] == '≈' {
                entity.basic_needs.hydrate = 10;
            }
            if board[y_usize][x_usize] == '+' {
                entity.basic_needs.starve = 10;
                board[y_usize][x_usize] = '#';
                return true;
            }
            if board[y_usize][x_usize] == 'ö' {
                entity.health -= 1;
            } 
            if board[y_usize][x_usize] == '#' {
                return true;
            }
        }
    }
    return false;
}
////////////////
//User actions//
////////////////
pub fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
        }
    }   
}
pub fn handle_input(usr_input: char, board: &mut Board, entity: &mut Entity) {
    let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
    if move_possibilites(board, usr_input, &coordinates, entity) {
        match usr_input {
            'w' => move_up(coordinates ,board),
            'a' => move_left(coordinates, board),
            's' => move_down(coordinates, board),
            'd' => move_right(coordinates, board),
            _ => println!("Error")
        }
    }
}
////////////////////
//Preditor actions//
////////////////////
pub fn move_preditor(board: &mut Board, entity: &mut Entity) {
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
            if move_possibilites(board, 'a', &co_preditor, entity) {
                move_left(co_preditor, board);
            }
        }
        else if resultx < 0 {
            if move_possibilites(board, 'd', &co_preditor, entity) {
                move_right(co_preditor, board);
            }
        }
        else if resulty < 0 {
            if move_possibilites(board, 's', &co_preditor, entity) {
                move_down(co_preditor, board);
            }
        }
        else if resulty > 0 {
            if move_possibilites(board, 'w', &co_preditor, entity) {
                move_up(co_preditor, board);
            }
        }
     }
}