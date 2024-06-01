use crate::{
  utils::{get_rdm_yx, is_inside_the_grid, find_char_in_board}, 
  world::{Board, Block}};
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
  let yx = get_rdm_yx(board);
  let yx_space = vec![yx[0] + 2, yx[1] + 2];
  if is_inside_the_grid(board, yx[1], yx[0])
  && is_inside_the_grid(board, yx_space[1], yx_space[0]) {
      for y in 0..3 {
          for x in 0..3 {
              board[yx[0] + y][yx[1] + x] = Block::new_predefined_set()[0].clone();
          }
      }
      true
  } else {
      false
  }
}
pub fn add_radom_water(board: &mut Board) {
  let yx:Vec<usize> = get_rdm_yx(board);
  let yx_space: Vec<usize> = vec![yx[0] + 1, yx[1] + 1];
  if is_inside_the_grid(board, yx[1], yx[0])
  && is_inside_the_grid(board, yx_space[1], yx_space[0]) {
      for y in 0..2 {
          for x in 0..2 {
              board[yx[0] + y][yx[1] + x] = Block::new_predefined_set()[2].clone();
          }
      }
  }
}
pub fn add_radom_food(board: &mut Board) {
  let yx:Vec<usize> = get_rdm_yx(board);
  let yx_space:Vec<usize> = vec![yx[0] + 3, yx[1] + 3];
  if is_inside_the_grid(board, yx[1], yx[0])
  && is_inside_the_grid(board, yx_space[1], yx_space[0]) {
          board[yx[0] + 2][yx[1]] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 2][yx[1] + 1] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 2][yx[1] + 2] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 2][yx[1] + 3] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 1][yx[1] + 1] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 1][yx[1] + 2] = Block::new_predefined_set()[4].clone();
          board[yx[0] + 3][yx[1] + 1] = Block::new_predefined_set()[3].clone();
          board[yx[0] + 3][yx[1] + 2] = Block::new_predefined_set()[3].clone();
      }
  }
pub fn add_cave(board: &mut Board) {
  let c:Vec<usize> = get_rdm_yx(board);
  board[c[0]][c[1]] = Block::new_predefined_set()[7].clone()
}
pub fn add_cave_exit(board: &mut Board) {
  let xy = find_char_in_board(board, '@');
  let x = xy[0];
  let y = xy[1];
  board[y][x+1] = Block::new_predefined_set()[7].clone()
  
}