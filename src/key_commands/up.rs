use crate::game_logic::Player;
use super::Command;

pub struct UpCommand {
  backup: String,
}
impl Command for UpCommand {
  fn execute(player: &mut Player) -> bool {
      player.y -= 1;
      return true
  }

  fn undo(player: &mut Player) {
      player.y += 1;
  }
}
