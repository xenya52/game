use crate::game_logic::{Entity, dead_entity};
//////////////////////
//General game logic//
//////////////////////
pub fn game_over(input: char, player: Entity) -> bool {
  if input == 'q' {
      return true;
  }
  else if dead_entity(player) {
      return true;
  }
  return  false;
}