use std::vec;

use crate::world::Board;
//////////////////////
///External imports///
//////////////////////
use rand::Rng;

//////////////////////////
//Get or set coordinates//
//////////////////////////
pub fn get_rdm_yx(board: &mut Board) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(2..board[1].len() - 2);
    let y: usize = rng.gen_range(2..board.len() - 2);
    return vec![y,x]
}

pub fn find_char_in_board(board: &Board, given: char) -> Vec<usize> {
    let mut coordinates = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char == given {
                coordinates.push(x);
                coordinates.push(y)
            }
        }
    }
    coordinates
}

pub fn is_inside_the_grid(board: &mut Board, x: usize, y: usize) -> bool {
    if y >= board.len()
    || x >= board[y].len(){
        false
    }
    else {
        true
    }
}