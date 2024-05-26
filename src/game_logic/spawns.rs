use crate::{
  world::Board, 
  utils::get_rdm_yx};

pub fn place_minion(board: &mut Board) {
  let xy:Vec<usize> = get_rdm_yx(board);
  board[xy[0]][xy[1]] = '@';
}