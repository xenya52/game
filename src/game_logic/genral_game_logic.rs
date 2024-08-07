use crate::game_logic::{Entity, dead_entity};

pub fn game_over(input: char, player: Entity) -> bool {
  if input == 'q' {
    return true;
  }
  else if dead_entity(player) {
    return true;
  }
  return  false;
}