
use rand::{thread_rng, Rng, seq::SliceRandom};
//Own stuff lib.rs
use crate::Board;

//////////////////////////
//Get or set coordinates//
//////////////////////////
pub fn get_rdm_xy(board: &mut Board) -> Vec<usize> {
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

pub fn find_char_in_board(board: &mut Board, given: char) -> Vec<(u32, u32)> {
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