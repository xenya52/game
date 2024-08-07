mod chunks; //Every predefined worldpart
mod terrain; //Parts of the world, like river or mountians
mod world; //basic parts of the world

pub use chunks::{init_cave, init_overworld};
pub use world::{Board, World, Block};
pub use terrain::{add_random_mountain, add_radom_food, 
  add_radom_water, add_cave, add_cave_exit,
  add_border, remove_border};