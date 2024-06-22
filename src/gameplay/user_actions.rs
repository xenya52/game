use crate::game_logic::{Entity, Player, Displaying, MoveDirections};
use crate::world::World;
use crate::gameplay::{movement_actions, inventory_actions, move_down, move_left, move_up, move_right};
//////////////////////
///External imports///
//////////////////////
pub use rand::{thread_rng, Rng, seq::SliceRandom};
use crossterm::{event::{read, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode}};
////////////////
//User actions//
////////////////
pub fn get_user_input() -> char {
  enable_raw_mode().expect("Failed to enable raw mode");

  let input: char;
  loop {
      if let Event::Key(key_event) = read().expect("Failed to read event") {
          match key_event.code {
              KeyCode::Char(c) => {
                  input = c;
                  break;
              }
              _ => continue,
          }
      }
  }

  disable_raw_mode().expect("Failed to disable raw mode");
  input
}
pub fn handle_input(player: &mut Player, world: &mut World, entity: &mut Entity) {
  if player.display_state == Displaying::Inventory {
    inventory_actions(player, entity, world);
  }
  else if player.current_entity == "Nothing" {
    match player.last_input {
      'w' => Player::movement(player, MoveDirections::Up),
      'a' => Player::movement(player, MoveDirections::Left),
      's' => Player::movement(player, MoveDirections::Down),
      'd' => Player::movement(player, MoveDirections::Right),
      'r' => Player::control_being(player, entity),
      _ => println!("Error: Invalid input in handle_input"),
    }
  }
  else {
    if movement_actions(world, player, entity) {
      match player.last_input {
        'w' => move_up(world, player),
        'a' => move_left(world, player),
        's' => move_down(world, player),
        'd' => move_right(world, player),
        _ => println!("Error"),
      }
    }
  }
  Player::player_made_turn(player);
  Player::debug(player);
}
