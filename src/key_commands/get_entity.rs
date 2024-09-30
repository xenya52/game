use crate::game_logic::{ Player, Displaying };
use super::Command;
pub struct GetEntityCommand {
  backup: String,
}
// impl Command for GetEntityCommand {
//   fn execute(&mut self, player: &mut Player, name: String) -> bool {
//       player.control_being(name);
//       return true
//   }

//   fn undo(&mut self, player: &mut Player) {
//       player.current_entity.name == "empty"
//   }
// }