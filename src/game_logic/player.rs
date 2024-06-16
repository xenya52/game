use crate::{utils::{find_char_in_board, get_board}, world::{Block, Board, World}};
/////////////////
//Print a board//
/////////////////
use std::fmt;

pub enum MoveDirections {
  Up,
  Down,
  Left,
  Right,
}
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
  pub turns: usize,
  pub y: usize,
  pub x: usize,
}
impl Player {
  pub fn new(_y: usize, _x: usize) -> Self {
    Player {
      last_input: 'E',
      display_state: Displaying::Overworld,
      last_display_state: Displaying::Overworld,
      turns: 0,
      y: _y,
      x: _x,
    }
  }
  pub fn movement(player: &mut Player, movement: MoveDirections) {
    match movement {
      MoveDirections::Up => player.y -= 1,
      MoveDirections::Down => player.y += 1,
      MoveDirections::Left => player.x -= 1,
      MoveDirections::Right => player.x += 1,
    }
  }
  pub fn player_made_turn(player: &mut Player) {
    player.turns += 1;
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
