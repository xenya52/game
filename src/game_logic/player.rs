use crate::world::{Block, World};

#[derive(PartialEq, Eq, Clone)]
pub enum Displaying {
  Overworld,
  Cave,
  Inventory,
}
pub struct Player {
  pub last_display_state: Displaying,
  pub display_state: Displaying,
  pub last_input: char,
}
impl Player {
  pub fn new() -> Self {
    Player {
      last_input: 'E',
      display_state: Displaying::Overworld,
      last_display_state: Displaying::Overworld,
    }
  }
  pub fn change_displaying_state(player: &mut Player, world: &mut World, y: usize, x: usize) {
    player.last_display_state = player.display_state.clone();
    let cave_entrance_block = Block::new_predefined_set()[7].clone();

    if player.last_input == 'i' && player.display_state != Displaying::Inventory {
      player.display_state = Displaying::Inventory;
    }
    else {
      println!("DEBUG ELSE!!!");
      match player.display_state {
        Displaying::Overworld => {
          println!("!!!");
          if world.overworld[y][x] == cave_entrance_block {
            player.display_state = Displaying::Cave;
          }
        }
        Displaying::Cave => {
        println!("!!!");
          if world.cave[y][x] == cave_entrance_block {
            player.display_state = Displaying::Overworld;
          }
        }
        Displaying::Inventory => {
          println!("!!!");
          player.display_state = player.last_display_state.clone();
        }
      }
    }
  }
}
