use crate::game_logic::Player;
use super::Command;

pub struct DownCommand {
  backup: String,
}
impl Command for DownCommand {
  fn execute(player: &mut Player) -> bool {
      player.y += 1;
      return true
  }

  fn undo(player: &mut Player) {
      player.y -= 1;
  }
}
