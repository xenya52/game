mod display; //Stuff to display various things in the world
mod entity; //Everything what has todo with a single entity and showing them
mod genral_game_logic; //Generl game logic functions
mod materials; //All materials that u can use to place blocks, or craft items
mod spawns; //Stuff to spawn entitys

pub use display::print_given_board;
pub use entity::{Entity, BasicNeeds, dead_entity, entity_moved};
pub use genral_game_logic::game_over;
pub use materials::Material;
pub use spawns::place_minion;