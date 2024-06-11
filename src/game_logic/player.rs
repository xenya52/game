#[derive(PartialEq, Eq)]
pub enum Displaying {
  Overworld,
  Cave,
  Inventory,
}
pub struct Player {
  pub display_state: Displaying
}
impl Player {
  pub fn new() -> Self {
    Player {
      display_state: Displaying::Overworld,
    }    
  }
}
