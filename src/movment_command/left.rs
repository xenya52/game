use crate::game_logic::Player;

pub struct LeftCommand {
  backup: String,
}
// https://refactoring.guru/design-patterns/command/rust/example
impl Command for LeftCommand {
  fn execute(&mut self, player: &mut Player) -> bool {
      player.x -= 1;
      return true
  }

  fn undo(&mut self, player: &mut Player) {
      player.x += 1;
  }
}
