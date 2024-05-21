pub fn change_world_state(world: &mut World) {
  if world.is_on_overworld {
      world.is_on_overworld = false;
      world.cave = init_cave();
  }
  else {
      world.is_on_overworld = true;
  }
}