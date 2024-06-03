use std::usize;

use crate::world::{init_overworld, Board, Block, World};

pub fn rezize_overworld_event(world: &mut World, tick: u32, curr_number: u32) {
  let cur_y_size = world.overworld.len() - 1;
  let cur_x_size = world.overworld[0].len() - 1;
  let board: Board = init_overworld(cur_y_size, cur_x_size);
  
  let new_y_size: usize = if cur_y_size < cur_x_size {cur_y_size * 2} else {cur_y_size};
  let new_x_size: usize = if cur_x_size < cur_y_size {cur_x_size * 2} else {cur_x_size};
  let mut updated_overworld: Board = vec![
                                     vec![Block::new_predefined_set()[5].clone(); 
                                     new_x_size]; new_y_size];

  for y in 0..new_y_size {
    for x in 0..new_x_size {
      println!("x = {} / y = {} || new_x_size = {} / new_y_size = {}", x, y, new_x_size, new_y_size);
      if cur_x_size < new_x_size && cur_x_size < x {
        println!("Use new board with x");
        updated_overworld[y][x] = board[y][x - cur_x_size].clone();
      }
      else if cur_y_size < new_y_size && cur_y_size < y {
      println!("Use new board with y");
        updated_overworld[y][x] = board[y - cur_y_size][x].clone();
      }
      else {
        println!("Use default overworld");
        updated_overworld[y][x] = world.overworld[y][x].clone();
      }
    }
  }
  world.overworld = updated_overworld;
} 
