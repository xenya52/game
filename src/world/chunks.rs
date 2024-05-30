use crate::utils::get_rdm_yx;
use crate::world::{
    Board,
    add_radom_food, add_radom_water, add_random_mountain, 
    add_cave, set_frame_in_board, add_cave_exit};
use crate::game_logic::place_minion;

use super::world::{BBoard, Block, Block_Type};

pub fn init_overworld() -> Board {
  let mut board = vec![vec!['#';32];16];
  let mut count = 0;
  //Generate barriers
  print!("Generate mountians ... ");
  loop {
      count += 1;
      add_random_mountain(&mut board);
      if count == 10 {
          break;
      }
  }
  count = 0;
  println!("Done!");
  //Generate water
  print!("Generate water ... ");
  loop {
      count += 1;
      add_radom_water(&mut board);
      if count == 3 {
          break;
      }
  }
  count = 0;
  println!("Done!");
  //Generate food
  print!("Generate food ... ");
  loop {
      count += 1;
      add_radom_food(&mut board);
      if count == 4 {
          break;
      }
  }
  println!("Done!");
  //Generate cave entrance
  print!("Generate cave entrance ...");
  add_cave(&mut board);
  println!("Done!");
  //Set player in board
  print!("Set one minion (Debug) in board ... ");
  place_minion(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  set_frame_in_board(&mut board);
  println!("Done!");
  return board;
}

pub fn init_cave() -> Board {
  let mut board = vec![vec!['#';32];16];
  let mut count = 0;
  //Generate barriers
  print!("Generate rock ... ");
  loop {
      count += 1;
      add_random_mountain(&mut board);
      if count == 10 {
          break;
      }
  }
  count = 0;
  println!("Done!");
  //Generate water
  print!("Generate water ... ");
  loop {
      count += 1;
      add_radom_water(&mut board);
      if count == 3 {
          break;
      }
  }
  println!("Done!");
  //Set player in cave
  print!("Set one minion (debug) in board ... ");
  place_minion(&mut board);
  println!("Done!");
  //Set cave exit
  print!("Generate cave exit ...");
  add_cave_exit(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  set_frame_in_board(&mut board);
  println!("Done!");
  return board;
}

pub fn init_bboard(x_size: usize, y_size: usize) -> BBoard {
    let mut a: Vec<Vec<Block>> = vec![vec![
                Block::new_predefined_set()[5].clone();x_size
                ];y_size
            ];
    bboard_border(&mut a);
    return a
}
fn bboard_border(given_board: &mut BBoard) {
    let y_len: usize = given_board.len();
    let x_len: usize = given_board[0].len();
    for (y, row) in given_board.clone().iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            if x >= x_len - 1 || x <= 0 || y >= y_len - 1 || y <= 0 {
                //Setting x border
                given_board[y][x] = Block::new_predefined_set()[1].clone()
            }
        }
    }
}

pub fn bboard_print(mut given_board: BBoard) {
    for (y, row) in given_board.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            print!("{}", col.display_ascii);
        }
        println!();
    }
}