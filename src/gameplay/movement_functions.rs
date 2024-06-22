use crate::game_logic::{Player, Entity, Displaying};
use crate::world::{Block, World};
use crate::utils::get_board;
//////////////////////////////
//General movement functions//
//////////////////////////////
pub fn move_up(world: &mut World, player: &Player) {
  let y = player.y;
  let x = player.x;
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
pub fn move_down(world: &mut World, player: &Player) {
  let y = player.y;
  let x = player.x;
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

pub fn move_right(world: &mut World, player: &Player) {
  let y = player.y;
  let x = player.x;
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

pub fn move_left(world: &mut World, player: &Player) {
  let y = player.y;
  let x = player.x;
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
pub fn inventory_actions(player: &mut Player, entity: &mut Entity, world: &mut World) {
  let y = player.y;
  let x = player.x;
  let index = entity.inventory.index - 1;
  match player.last_input {
    'd' => {
      if 0 < index {
        entity.inventory.index -= 1;
      }
    }
    'a' => {
      if entity.inventory.items.len() - 1 > index {
        entity.inventory.index += 1;
      }
    }
    'c' => {
      print!("{}", entity.inventory.items[index]);
      if entity.inventory.items[index] == "Food" {
        entity.inventory.items[index] = "---".to_string();
        entity.basic_needs.starve = 10;
      } 
      else if entity.inventory.items[index] == "Water" {
        entity.inventory.items[index] = "---".to_string();
        entity.basic_needs.hydrate = 10;
      }
    }
    _ => {Player::change_displaying_state(player, get_board(world, player.last_display_state.clone()), y, x)}
  }
}

pub fn movement_actions(world: &mut World, player: &mut Player, entity: &mut Entity) -> bool {
  let mut y = player.y;
  let mut x = player.x;
  match player.last_input {
      'w' => y -= 1,
      'a' => x -= 1,
      's' => y += 1,
      'd' => x += 1,
      _ => println!("Error")
  }
  let board = get_board(world, player.last_display_state.clone());
  
  Player::change_displaying_state(player, board, y, x);
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
