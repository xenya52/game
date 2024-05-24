use crate::world::{Board, World, change_world_state};
use crate::game_logic::{Entity};

pub fn ascii_definitions(world: &mut World, x:usize, y:usize, entity: &mut Entity) -> bool {
  //Action for stone
  if world.overworld[y][x] == 'x' || world.overworld[y][x] == 'X' {
    entity.materials.stone += 1;
  }
  //Action for wood
  if world.overworld[y][x] == '|' {
      entity.materials.wood += 1;
      world.overworld[y][x] = '#';
      return true;
  }
  //Action for water
  if world.overworld[y][x] == '~' || world.overworld[y][x] == '≈' {
      entity.basic_needs.hydrate = 10;
  }
  //Action for food
  if world.overworld[y][x] == '+' {
      entity.basic_needs.starve = 10;
      world.overworld[y][x] = '#';
      return true;
  }
  //Action for hitting the eniemy_entity
  if world.overworld[y][x] == 'ö' {
      entity.health -= 1;
  }
  if world.overworld[y][x] == 'o' {
      change_world_state(world);
      return true;
  }
  //Action for default grass
  if world.overworld[y][x] == '#' {
      return true;
  }
  return false;
}