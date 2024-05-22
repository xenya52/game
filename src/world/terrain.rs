mod utils;
use crate::world::{Board, World};

pub fn set_frame_in_board(board: &mut Board) {
  let row_count = board.len();
  let col_count = board.first().unwrap().len();

  //first and last rows
  for j in 0..col_count {
      board[0][j] = '/'; // first row
      board[row_count - 1][j] = '/'; // last row
  }

  //columns for rows in between
  for i in 1..(row_count - 1) {
      board[i][0] = '/'; // first colum
      board[i][col_count - 1] = '/'; // last column
  }
}

////////////////////
//World generation//
////////////////////
pub fn add_random_mountain(board: &mut Board) -> bool {
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
pub fn add_radom_water(board: &mut Board) {
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
pub fn add_radom_food(board: &mut Board) {
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
pub fn add_cave(board: &mut Board) {
  let c:Vec<usize> = get_rdm_xy(board);
  board[c[0]][c[1]] = 'o';
}