use crate::game_logic::Player;

pub struct UpCommand {
  backup: String,
}
// https://refactoring.guru/design-patterns/command/rust/example
impl Command for UpCommand {
  fn execute(&mut self, player: &mut Player) -> bool {
      player.y -= 1;
      return true
  }

  fn undo(&mut self, player: &mut Player) {
      player.y += 1;
  }
}
