use crate::world::{Board};

pub fn ascii_definitions(board: Board, x:usize, y:usize, entity: Entity) {
  //Action for stone
  if board[y][x] == 'x' || board[y][x] == 'X' {
    entity.materials.stone += 1;
  }
  //Action for wood
  if board[y][x] == '|' {
      entity.materials.wood += 1;
      board[y][x] = '#';
      return true;
  }
  //Action for water
  if board[y][x] == '~' || board[y][x] == '≈' {
      entity.basic_needs.hydrate = 10;
  }
  //Action for food
  if board[y][x] == '+' {
      entity.basic_needs.starve = 10;
      board[y][x] = '#';
      return true;
  }
  //Action for hitting the eniemy_entity
  if board[y][x] == 'ö' {
      entity.health -= 1;
  }
  if board[y][x] == 'o' {
      change_world_state(world);
      return true;
  }
  //Action for default grass
  if board[y][x] == '#' {
      return true;
  }
}