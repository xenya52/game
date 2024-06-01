# TODOS
- implement regrowable trees
- Make a "overworld size class and extend the overwold after a specific amount of turns"
- //TODO give items in movement functions, line 80
- Rework materials
- implement buildable structures
- TODO find MAX u32 function for Water
- Write a good md
- Rework your game loop
- Idea for worldstates
// pub enum WORLD_STATE {
//   is_on_overworld,
//   is_in_cave,
//   is_in_a_fight
// }

-----------

## Dev sitenodes

### Blocktype predefined set
!!! Important !!! if the val of BlockType.durability is 404 it means the http code so there is no durability
- Index 0 is unbreakable -> You wont get anything from there, unpassable, like borders or portals/cave_entrance
- Index 1 is unbreakable_recources -> Is intended to set a drop later, because it has a durability 
- Index 2 is breakable_resource -> Is intended to set a drop later, also should disapear
- Index 3 is water -> Yea...
- Index 4 is air -> Uncappable block to push around
### Block predefined set
- Index 0 is stone
- Index 1 is dirt
- Index 2 is water
- Index 3 is wood
- Index 4 is food
- Index 5 is air
- Index 6 is Borders
- Index 7 is Cave entrance
- Index 8 is Minion
### Materials predefined set
- Index 0 is nothing
- Index 1 is wood
- Index 2 is stone
- Index 3 is dirt
- Index 4 is water
- Index 5 is food