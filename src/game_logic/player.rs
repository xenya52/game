use crate::world::{Block, Board};
/////////////////
//Print a board//
/////////////////
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub enum Displaying {
  Overworld,
  Cave,
  Inventory,
}

impl fmt::Display for Displaying {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Displaying::Overworld => write!(f, "Overworld"),
           Displaying::Cave => write!(f, "Cave"),
           Displaying::Inventory => write!(f, "Inventory"),
       }
    }
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
  pub fn change_displaying_state(player: &mut Player, board: &mut Board, y: usize, x: usize) {
    let cave_entrance_block = Block::new_predefined_set()[7].clone();

    if player.last_input == 'i' {
      player.display_state = Displaying::Inventory;
    }
    else {
      match player.display_state {
        Displaying::Overworld => {
          if board[y][x] == cave_entrance_block {
            player.display_state = Displaying::Cave;
          }
        }
        Displaying::Cave => {
          if board[y][x] == cave_entrance_block {
            player.display_state = Displaying::Overworld;
          }
        }
        Displaying::Inventory => {
          player.display_state = player.last_display_state.clone();
        }
      }
    }
    println!("!!!! last {}, display_state {}", player.last_display_state, player.display_state);
    if player.last_display_state == player.display_state {
      player.last_display_state = player.display_state.clone();
    }
  }
}
