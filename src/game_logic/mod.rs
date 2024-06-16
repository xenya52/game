mod display; //Stuff to display various things in the world
mod entity; //Everything what has todo with a single entity and showing them
mod genral_game_logic; //Generl game logic functions
mod spawns; //Stuff to spawn entitys
mod player;

pub use display::{print_given_board, print_keybindings};
pub use entity::{Entity, BasicNeeds, dead_entity, entity_moved};
pub use genral_game_logic::game_over;
pub use spawns::place_minion;
pub use player::{Displaying, MoveDirections, Player};
