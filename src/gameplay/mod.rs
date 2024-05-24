mod movement_functions; //functions so the entitys can move
mod user_actions; //Interactions methods for the user

pub use user_actions::{get_user_input, handle_input};
pub use movement_functions::{movement_actions, move_down, move_left, move_up, move_right};