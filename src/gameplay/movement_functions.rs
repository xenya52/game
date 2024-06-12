use crate::game_logic::{Player, Displaying};
use crate::world::{Block, World};
use crate::Entity;
//////////////////////////////
//General movement functions//
//////////////////////////////
pub fn move_up(x: usize, y: usize, world: &mut World, player: &Player) {
  let &mut board;
  if player.display_state == Displaying::Overworld{
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y - 1][x].clone();
  board[y - 1][x] = board[y][x].clone();
  board[y][x] = temp;
}
pub fn move_down(x: usize, y: usize, world: &mut World, player: &Player) {
  let &mut board;
  if player.display_state == Displaying::Overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y + 1][x].clone();
  board[y + 1][x] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn move_right(x: usize, y: usize, world: &mut World, player: &Player) {
  let &mut board;
  if player.display_state == Displaying::Overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y][x + 1].clone();
  board[y][x + 1] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn move_left(x: usize, y: usize, world: &mut World, player: &Player) {
  let &mut board;
  if player.display_state == Displaying::Overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  let temp = board[y][x - 1].clone();
  board[y][x - 1] = board[y][x].clone();
  board[y][x] = temp;
}

pub fn movement_actions(world: &mut World, player: &mut Player, entity: &mut Entity, mut x: usize, mut y: usize) -> bool {
  Player::change_displaying_state(player, world, y, x);
  match player.last_input {
      'w' => y -= 1,
      'a' => x -= 1,
      's' => y += 1,
      'd' => x += 1,
      _ => println!("Error")
  }
  let &mut board;
  if player.display_state == Displaying::Overworld {
    board = &mut world.overworld;
  }
  else {
    board = &mut world.cave
  }
  //If the block is breakable and has a durability of 0 up
  if board[y][x].block_type.durability == 0 {
    Entity::block_to_inventory(entity, board[y][x].clone());
    if board[y][x].block_type.is_passable {
      board[y][x] = Block::new_predefined_set()[5].clone(); //Replace block with "air" should be passable
      return true;
    }
    else {
      board[y][x].block_type.durability += 4; // Reset durability because if its not passable but gives something it should be always 4
      return false;   
    }
  }
  //If the block is breakable and has durability
  else if board[y][x].block_type.durability != 404{
    board[y][x].block_type.durability -= 1;
    return false;
  }
  //If the block is just passable and gives nothing
  else if board[y][x].block_type.is_passable {
    return true;
  }
  else {
      return false
  }
}
