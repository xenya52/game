use crate::world::{init_overworld, add_border, remove_border, Block, Board, World};
//////////////////////
///External imports///
//////////////////////
use std::usize;

pub fn rezize_overworld_event(world: &mut World, tick: usize, curr_number: usize) {
  if curr_number % tick == 0 && curr_number != 0 { //curr number means like the turns and tick
                                                   //after how much moves something should happen
    let cur_y_size = world.overworld.len();
    let cur_x_size = world.overworld[0].len();
    let mut board: Board = init_overworld(cur_x_size, cur_y_size);
    
    remove_border(&mut world.overworld);
    remove_border(&mut board);

    let new_y_size: usize = if cur_y_size < cur_x_size {cur_y_size * 2} else {cur_y_size};
    let new_x_size: usize = if cur_x_size == cur_y_size {cur_x_size * 2} else {cur_x_size};
    let mut updated_overworld: Board = vec![
                                     vec![Block::new_predefined_set()[5].clone(); 
                                     new_x_size]; new_y_size];
    
    for y in 0..new_y_size {
      for x in 0..new_x_size {
        if cur_x_size < new_x_size && cur_x_size - 1 < x {
          updated_overworld[y][x] = board[y][x - cur_x_size].clone();
        }
        else if cur_y_size < new_y_size && cur_y_size - 1 < y {
        updated_overworld[y][x] = board[y - cur_y_size][x].clone();
        }
        else {
          updated_overworld[y][x] = world.overworld[y][x].clone();
        }
      }
    }
    add_border(&mut updated_overworld);
    world.overworld = updated_overworld;
  }
} 
