use crate::{
  world::Board, 
  utils::get_rdm_xy};

pub fn place_minion(board: &mut Board) {
  let xy:Vec<usize> = get_rdm_xy(board);
  board[xy[0]][xy[1]] = '@';
}