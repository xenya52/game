use crate::game_logic::{ Player, Displaying };
use super::Command;

pub struct LeftCommand {
  backup: String,
}
impl Command for LeftCommand {
  fn execute(&mut self, player: &mut Player) -> bool {
      player.last_display_state = player.display_state;
      player.display_state = Displaying::Inventory;
      return true
  }

  fn undo(&mut self, player: &mut Player) {
      player.display_state = player.last_display_state;
  }
}
