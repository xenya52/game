use crate::{
  utils::{get_rdm_yx, is_inside_the_grid, find_char_in_board}, 
  world::{Board, Block, World}};
//////////////////////
///External imports///
//////////////////////
use rand::{thread_rng, Rng, seq::SliceRandom};

pub fn add_border(given_board: &mut Board) {
  let y_len: usize = given_board.len();
    let x_len: usize = given_board[0].len();
    for y in 0..y_len {
        for x in 0..x_len {
            if x >= x_len - 1 || x <= 0 || y >= y_len - 1 || y <= 0 {
                //Setting x border
                given_board[y][x] = Block::new_predefined_set()[6].clone()
            }
        }
    }
}

////////////////////
//World generation//
////////////////////
pub fn add_random_mountain(board: &mut Board) -> bool {
  let c = get_rdm_yx(board);
  let c_space = vec![c[0] + 2, c[1] + 2];

  if is_inside_the_grid(board, c[1], c[0])
  && is_inside_the_grid(board, c_space[1], c_space[0]) {
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
  let c:Vec<usize> = get_rdm_yx(board);
  let c_space: Vec<usize> = vec![c[0] + 1, c[1] + 1];
  if is_inside_the_grid(board, c[1], c[0])
  && is_inside_the_grid(board, c_space[1], c_space[0]) {
      for i in 0..2 {
          for j in 0..2 {
              let value = '~';
              board[c[0] + i][c[1] + j] = value;
          }
      }
  }
}
pub fn add_radom_food(board: &mut Board) {
  let c:Vec<usize> = get_rdm_yx(board);
  let c_space:Vec<usize> = vec![c[0] + 3, c[1] + 3];
  let food: char = '+';
  let wood: char = '|';
  if is_inside_the_grid(board, c[1], c[0])
  && is_inside_the_grid(board, c_space[1], c_space[0]) {
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
  let c:Vec<usize> = get_rdm_yx(board);
  board[c[0]][c[1]] = 'o';
}
pub fn add_cave_exit(board: &mut Board) {
  let xy = find_char_in_board(board, '@');
  let x = xy[0];
  let y = xy[1];
  board[y][x+1] = 'o'
}