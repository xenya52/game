mod board;
mod utils;
mod game;

pub use board::{Board, init_board, print_board};
pub use game::{game_over, get_user_input, handle_input, move_preditor};
pub use utils::{get_rdm_xy, find_char_in_board};