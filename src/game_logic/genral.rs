//////////////////////
//General game logic//
//////////////////////
pub fn game_over(input: char, player: Entity) -> bool {
  if input == 'q' {
      return true;
  }
  else if dead_entity(player) {
      return true;
  }
  return  false;
}
/////////
//Board//
/////////
pub type Board = Vec<Vec<char>>;
pub struct World {
    pub overworld: Vec<Vec<char>>,
    pub cave: Vec<Vec<char>>,
    pub is_on_overworld: bool
}
impl World {
    pub fn new(_overworld: Vec<Vec<char>>, _cave: Vec<Vec<char>>) -> Self {
        World {
            overworld: _overworld,
            cave: _cave,
            is_on_overworld: true
        }
    }
}