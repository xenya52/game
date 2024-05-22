use super::world::Board;

pub fn ascii_definitions(board: Board) {
  //Action for stone
  if board[y_usize][x_usize] == 'x' || board[y_usize][x_usize] == 'X' {
    entity.materials.stone += 1;
  }
  //Action for wood
  if board[y_usize][x_usize] == '|' {
      entity.materials.wood += 1;
      board[y_usize][x_usize] = '#';
      return true;
  }
  //Action for water
  if board[y_usize][x_usize] == '~' || board[y_usize][x_usize] == '≈' {
      entity.basic_needs.hydrate = 10;
  }
  //Action for food
  if board[y_usize][x_usize] == '+' {
      entity.basic_needs.starve = 10;
      board[y_usize][x_usize] = '#';
      return true;
  }
  //Action for hitting the eniemy_entity
  if board[y_usize][x_usize] == 'ö' {
      entity.health -= 1;
  }
  if board[y_usize][x_usize] == 'o' {
      change_world_state(world);
      return true;
  }
  //Action for default grass
  if board[y_usize][x_usize] == '#' {
      return true;
  }
}