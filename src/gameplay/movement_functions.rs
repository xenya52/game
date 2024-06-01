use crate::world::World;
//////////////////////////////
//General movement functions//
//////////////////////////////
pub fn move_up(x: usize, y: usize, world: &mut World) {
  let &mut board;
  if world.is_on_overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y - 1][x].clone();
  board[y - 1][x] = board[y][x].clone();
  board[y][x] = temp;
}
pub fn move_down(x: usize, y: usize, world: &mut World) {
  let &mut board;
  if world.is_on_overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y + 1][x].clone();
  board[y + 1][x] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn move_right(x: usize, y: usize, world: &mut World) {
  let &mut board;
  if world.is_on_overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y][x + 1].clone();
  board[y][x + 1] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn move_left(x: usize, y: usize, world: &mut World) {
  let &mut board;
  if world.is_on_overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y][x - 1].clone();
  board[y][x - 1] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn movement_actions(world: &mut World, usr_input:char, mut x: usize, mut y: usize) -> bool {
  match usr_input {
      'w' => y -= 1,
      'a' => x -= 1,
      's' => y += 1,
      'd' => x += 1,
      _ => println!("Error")
  }
  let &mut board;
  if world.is_on_overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  return board[y][x].block_type.is_passable;
}
