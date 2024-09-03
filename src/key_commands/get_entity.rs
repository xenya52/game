use crate::game_logic::{ Player, Displaying };

pub struct LeftCommand {
  backup: String,
}
impl Command for LeftCommand {
  fn execute(&mut self, player: &mut Player, name: String) -> bool {
      player.control_being(name);
      return true
  }

  fn undo(&mut self, player: &mut Player) {
      player.current_entity.name == "empty"
  }
}