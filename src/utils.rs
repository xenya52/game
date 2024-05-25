use crate::world::Board;
//////////////////////
///External imports///
//////////////////////
use rand::{thread_rng, Rng, seq::SliceRandom};

//////////////////////////
//Get or set coordinates//
//////////////////////////
pub fn get_rdm_xy(board: &mut Board) -> Vec<usize> {
    let mut empty_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
                empty_tiles.push((i, j));
        }
    }

    if let Some(&(x, y)) = empty_tiles.choose(&mut thread_rng()) {
        return vec![x,y]
    }
    else {
        return vec![0,0]
    }
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

pub fn is_inside_the_grid(board: &mut Board, coordinates: &Vec<usize>) -> bool {
    if coordinates.len() != 2 {
        return false;
    }
    let y: usize = coordinates[0];
    let x: usize = coordinates[1];
    if y >= board.len()
    || x >= board[y].len(){
        false
    }
    else {
        true
    }
}