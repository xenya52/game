use crate::world::{Board, Block, World};
use crate::game_logic::Displaying;
//////////////////////
///External imports///
//////////////////////
use rand::{Rng, thread_rng};
use std::vec;
//////////////////////////
//Get or set coordinates//
//////////////////////////
pub fn get_rdm_yx(board: &mut Board) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(2..board[1].len() - 2);
    let y: usize = rng.gen_range(2..board.len() - 2);
    return vec![y,x]
}
pub fn choose_between_two_blocks(index_a: usize, index_b: usize) -> Block {
  let set = Block::new_predefined_set();
  return if thread_rng().gen_bool(0.75) { set[index_a].clone()} else { set[index_b].clone()};
}

pub fn find_char_in_board(given_board: &Board, given: char) -> Vec<usize> {
    let y_len: usize = given_board.len();
    let x_len: usize = given_board[0].len();

    let mut coordinates = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if given_board[y][x].display_ascii == given {
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
pub fn get_board(world: &mut World, last_display: Displaying) -> &mut Board {
  if last_display == Displaying::Overworld {
    return &mut world.overworld;
  }
  else {
    return &mut world.cave;
  }
}
