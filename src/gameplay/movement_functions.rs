//////////////////////
///External imports///
//////////////////////
pub use rand::{thread_rng, Rng, seq::SliceRandom};
use crossterm::{
  event::{read, Event, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode}
};

//////////////////////////////
//General movement functions//
//////////////////////////////
fn move_up(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
  if let Some((x, y)) = coordinates.get(0) {
      let x_usize = *x as usize;
      let y_usize = *y as usize;

      // Swap the character with the element above it
      let temp = board[y_usize - 1][x_usize];
      board[y_usize - 1][x_usize] = board[y_usize][x_usize];
      board[y_usize][x_usize] = temp;
  }
}
fn move_down(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
  if let Some((x, y)) = coordinates.get(0) {
      let x_usize = *x as usize;
      let y_usize = *y as usize;

      let temp = board[y_usize + 1][x_usize];
      board[y_usize + 1][x_usize] = board[y_usize][x_usize];
      board[y_usize][x_usize] = temp;
  }
}

fn move_right(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
  if let Some((x, y)) = coordinates.get(0) {
      let x_usize = *x as usize;
      let y_usize = *y as usize;

      let temp = board[y_usize][x_usize + 1];
      board[y_usize][x_usize + 1] = board[y_usize][x_usize];
      board[y_usize][x_usize] = temp;
  }
}

fn move_left(coordinates: Vec<(u32, u32)>, board: &mut Vec<Vec<char>>) {
  if let Some((x, y)) = coordinates.get(0) {
      let x_usize = *x as usize;
      let y_usize = *y as usize;

      let temp = board[y_usize][x_usize - 1];
      board[y_usize][x_usize - 1] = board[y_usize][x_usize];
      board[y_usize][x_usize] = temp;
  }
}

fn movement_actions(world: &mut World, usr_input:char, coordinates: &Vec<(u32, u32)>, entity: &mut Entity) -> bool {
  let mut board: Board = world.overworld;
  if world.is_on_overworld {
      board = world.overworld
  }
  else {
      board = world.cave
  }
  if let Some((x, y)) = coordinates.get(0) {
      if *x > 0 {
          let mut x_usize = *x as usize;
          let mut y_usize = *y as usize;
          
          match usr_input {
              'w' => y_usize -= 1,
              'a' => x_usize -= 1,
              's' => y_usize += 1,
              'd' => x_usize += 1,
              _ => println!("Error")
          }
          
      }
  }
  return false;
}