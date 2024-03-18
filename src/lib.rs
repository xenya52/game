mod board;
mod utils;
mod game;

pub use board::{Board, init_board, print_board}; //Board functions
pub use game::{game_over}; //Gamerules
pub use game::{Entity, new_entity, show_entity_status, entity_moved}; //Entity stuff
pub use game::{get_user_input, handle_input, move_preditor}; //User and preditor actions
pub use utils::{get_rdm_xy, find_char_in_board, is_inside_the_grid}; //Handie / Commonly used  function