use crate::world::{Board, World, ascii_definitions};
use crate::game_logic::Entity;
//////////////////////
///External imports///
//////////////////////
use crossterm::{
  event::{read, Event, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode}
};

//////////////////////////////
//General movement functions//
//////////////////////////////
pub fn move_up(x: usize, y: usize, world: &mut World) {
  let mut board: Board;
  if world.is_on_overworld {
    board = world.overworld;
  }
  else {
    board = world.cave
  }
  let temp = board[y - 1][x];
  board[y - 1][x] = board[y][x];
  board[y][x] = temp;
}
pub fn move_down(x: usize, y: usize, world: &mut World) {
  let mut board: Board;
  if world.is_on_overworld {
    board = world.overworld;
  }
  else {
    board = world.cave
  }
  let temp = board[y + 1][x];
  board[y + 1][x] = board[y][x];
  board[y][x] = temp;
}

pub fn move_right(x: usize, y: usize, world: &mut World) {
  let mut board: Board;
  if world.is_on_overworld {
    board = world.overworld;
  }
  else {
    board = world.cave
  }
  let temp = board[y][x + 1];
  board[y][x + 1] = board[y][x];
  board[y][x] = temp;
}

pub fn move_left(x: usize, y: usize, world: &mut World) {
  let mut board: Board;
  if world.is_on_overworld {
    board = world.overworld;
  }
  else {
    board = world.cave
  }
  let temp = board[y][x - 1];
  board[y][x - 1] = board[y][x];
  board[y][x] = temp;
}

pub fn movement_actions(world: &mut World, usr_input:char, mut x: usize, mut y: usize, entity: &mut Entity) -> bool {
  let mut board: Board = world.overworld;
  if world.is_on_overworld {
      board = world.overworld
  }
  else {
      board = world.cave
  }
  match usr_input {
      'w' => y -= 1,
      'a' => x -= 1,
      's' => y += 1,
      'd' => x += 1,
      _ => println!("Error")
  }
  return ascii_definitions(world,x,y,entity);
}