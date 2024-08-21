
pub struct UpCommand {
  backup: String,
}
// https://refactoring.guru/design-patterns/command/rust/example
impl Command for UpCommand {
  fn execute(&mut self, player: &mut Player) -> bool {
      let mut movement = player.y;

      app.with_user_data(|context: &mut AppContext| {
          self.backup = editor.get_content().to_string();
          context.clipboard = self.backup.clone();
          editor.set_content("".to_string());
      });

      true
  }

  fn undo(&mut self, app: &mut Cursive) {
      let mut editor = app.find_name::<EditView>("Editor").unwrap();
      editor.set_content(&self.backup);
  }
}
