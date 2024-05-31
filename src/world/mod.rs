mod ascii_definitions; //Defines every block
mod states; //Current worlds states
mod chunks; //Every predefined worldpart
mod terrain; //Parts of the world, like river or mountians
mod world; //basic parts of the world

pub use terrain::{add_random_mountain, add_radom_food, 
  add_radom_water, add_cave, add_cave_exit,
  add_border};
// pub use ascii_definitions::ascii_definitions; Guess its out
pub use chunks::{init_cave, init_overworld, init_board};
pub use states::change_world_state;
pub use world::{Board, World, Block, Block_Type};