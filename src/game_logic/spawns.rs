use crate::{ 
    utils::get_rdm_yx, 
    world::{Block, Board}
};

pub fn place_minion(board: &mut Board) {
  let xy:Vec<usize> = get_rdm_yx(board);
  board[xy[0]][xy[1]] = Block::new_predefined_set()[8].clone();
}
