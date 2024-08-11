use crate::world::{Board, Block, World};
use crate::game_logic::{Displaying, Player};
//////////////////////
///External imports///
//////////////////////
use rand::{Rng, thread_rng};
use std::vec;
//////////////////////////
//Get or set coordinates//
//////////////////////////
pub fn get_rdm_yx(board: &mut Board) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(2..board[1].len() - 2);
    let y: usize = rng.gen_range(2..board.len() - 2);
    return vec![y,x]
}
pub fn choose_between_two_blocks(index_a: usize, index_b: usize) -> Block {
  let set = Block::new_predefined_set();
  return if thread_rng().gen_bool(0.75) { set[index_a].clone()} else { set[index_b].clone()};
}

pub fn find_char_in_board(given_board: &Board, given: char) -> Vec<usize> {
    let y_len: usize = given_board.len();
    let x_len: usize = given_board[0].len();

    let mut coordinates = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if given_board[y][x].display_ascii == given {
                coordinates.push(x);
                coordinates.push(y)
            }
        }
    }
    coordinates
}

pub fn is_inside_the_grid(board: &mut Board, x: usize, y: usize) -> bool {
    if y >= board.len()
    || x >= board[y].len(){
        false
    }
    else {
        true
    }
}
pub fn get_board(world: &mut World, last_display: Displaying) -> &mut Board {
  if last_display == Displaying::Overworld {
    return &mut world.overworld;
  }
  else {
    return &mut world.cave;
  }
}
pub fn create_rendered_board(board: &mut Board, player: &mut Player, render_distance: usize) -> Board {
  println!("Run into the create_rendered_board function!");
  let mut rendered_board: Vec<Vec<Block>> = vec![
                                     vec![
                                       Block::new_predefined_set()[5].clone();
                                       render_distance
                                     ];
                                   render_distance
                                   ];
  let start_x: usize;
  if player.x > render_distance / 2 {
    if player.x - render_distance / 2 > board[0].len() {
      start_x = board[0].len();
    }
    else {
        start_x = player.x - render_distance / 2 + 1
    }
  }
  else {
    start_x = 0;
  }
  let start_y: usize;
  if player.y > render_distance / 2 {
    if player.y - render_distance / 2 > board[0].len() {
      start_y = board[0].len();
    }
    else {
        start_y = player.y - render_distance / 2 + 1
    }
  }
  else {
    start_y = 0;
  }
  println!("Player X = {}", player.x);
  println!("Player Y = {}", player.y);
  println!("Start X = {}", start_x);
  println!("Start Y = {}", start_y);
  for y in 0..render_distance {
    for x in 0..render_distance {
      rendered_board[y][x] = board[start_y + y][start_x + x].clone();
    }
  }
  return rendered_board;
}