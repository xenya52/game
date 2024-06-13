use crate::game_logic::{Entity, Player, Displaying};
use crate::utils::find_char_in_board;
use crate::world::World;
use crate::gameplay::{movement_actions, move_down, move_left, move_up, move_right};
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
              }
              _ => continue,
          }
      }
  }

  disable_raw_mode().expect("Failed to disable raw mode");
  input
}
pub fn handle_input(player: &mut Player, world: &mut World, entity: &mut Entity) {
  let xy: Vec<usize>;
  if player.display_state == Displaying::Overworld {
    xy = find_char_in_board(&world.overworld, '@');
  }
  else {
    xy = find_char_in_board(&world.cave, '@');
  }
  let x: usize = xy[0];
  let y: usize = xy[1];
    
  if movement_actions(world, player, entity, x, y) {  
      match player.last_input {
          'w' => move_up(x, y, world, player),
          'a' => move_left(x, y, world, player),
          's' => move_down(x, y, world, player),
          'd' => move_right(x, y, world, player),
          'i' => player.display_state = Displaying::Inventory,
          _ => println!("Error"),
      }
  }
}
