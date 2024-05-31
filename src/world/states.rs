use crate::world::{World, init_cave};
pub enum WORLD_STATE {
  is_on_overworld,
  is_in_cave,
  is_in_a_fight
}

pub fn change_world_state(world: &mut World) {
  if world.is_on_overworld {
      world.is_on_overworld = false;
      world.cave = init_cave(32,16); //Todo make flexible size
  }
  else {
      world.is_on_overworld = true;
  }
}