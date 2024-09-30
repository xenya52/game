use crate::game_logic::{Entity, Player, Displaying, MoveDirections};
use crate::world::World;
use crate::gameplay::{movement_actions, inventory_actions};
use crate::key_commands::{Command, DownCommand, UpCommand, LeftCommand, RightCommand};
//////////////////////
///External imports///
//////////////////////
use crossterm::{event::{read, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode}};

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
  else if player.current_entity.name == "empty" {
    match player.last_input {
      'w' => Player::movement(player, MoveDirections::Up),
      'a' => Player::movement(player, MoveDirections::Left),
      's' => Player::movement(player, MoveDirections::Down),
      'd' => Player::movement(player, MoveDirections::Right),
      'r' => Player::control_being(player, entity),
      _ => println!("DebugError: Invalid input in handle_input, spectator move"),
    }
  }
  else {
    if movement_actions(world, player, entity) {
      match player.last_input {
        'w' => Command::execute(&mut player),
        'a' => Command::execute(&mut player),
        's' => Command::execute(&mut player),
        'd' => Command::execute(&mut player),
        // 'r' => player.current_entity.name = "empty".to_string(),
        // _ => println!("DebugError: Invalid input in handle_input control being, while moving entity"),
      }
    }
    else {
      match player.last_input {
        'r' => player.current_entity.name = "empty".to_string(),
        _ => println!("DebugError: Invalid input in handle_input control being")
      }
    }
  }
  Player::player_made_turn(player);
}
