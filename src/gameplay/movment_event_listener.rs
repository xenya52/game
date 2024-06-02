use std::usize;

use crate::world::{init_overworld, Board, Block World};

pub fn rezize_overworld_event(world: &mut World, tick: u32, curr_number: u32) {
  let cur_y_size = world.overworld.len();
  let cur_x_size = world.overworld[0].len();
  let board: Board = init_overworld(cur_y_size, cur_x_size);
  
  let new_y_size: usize;
  let new_x_size: usize;
  if cur_y_size > cur_x_size {
    new_x_size *= 2;
  }
  else {
    new_y_size *= 2;
  }
  let mut updated_overworld: Board = vec![
                                     vec![Block::new_predefined_set()[5]; 
                                     new_x_size]; new_y_size];

  for y in 0..new_y_size {
    for x in 0..new_x_size {
      if cur_x_size < new_x_size && cur_x_size < x {
        updated_overworld[y][x] = board[y][x - cur_x_size] 
      }
      else if cur_y_size < new_y_size && cur_y_size < y {
        updated_overworld[y][x] = board[y - cur_y_size][x]
      }
      else {
        updated_overworld[y][x] = world.overworld[y][x]
      }
    }
  }
  world.overworld = updated_overworld;
} 
