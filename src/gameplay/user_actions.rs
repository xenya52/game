//////////////////////
///External imports///
//////////////////////
pub use rand::{thread_rng, Rng, seq::SliceRandom};

////////////////
//User actions//
////////////////
pub fn get_user_input() -> char {
  enable_raw_mode().expect("Failed to enable raw mode");

  let mut input_char = ' ';
  loop {
      if let Event::Key(key_event) = read().expect("Failed to read event") {
          match key_event.code {
              KeyCode::Char(c) => {
                  input_char = c;
                  break;
              }
              _ => continue,
          }
      }
  }

  // Deaktiviere den Raw-Modus, bevor das Programm endet
  disable_raw_mode().expect("Failed to disable raw mode");
  input_char
}
pub fn handle_input(usr_input: char, world: &mut World, entity: &mut Entity) {
  let board: &mut Board;
  if world.is_on_overworld {
      board = &mut world.overworld;
  }
  else {
      board = &mut world.cave
  }
  let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
  // if move_possibilites(&mut World::clone(&world), usr_input, &coordinates, entity) {
  if true {    
      match usr_input {
          'w' => move_up(coordinates ,board),
          'a' => move_left(coordinates, board),
          's' => move_down(coordinates, board),
          'd' => move_right(coordinates, board),
          _ => println!("Error")
      }
  }
}