use crate::game_logic::Player;
use super::Command;

pub struct LeftCommand {
  backup: String,
}
impl Command for LeftCommand {
  fn execute(player: &mut Player) -> bool {
      player.x -= 1;
      return true
  }

  fn undo(player: &mut Player) {
      player.x += 1;
  }
}
