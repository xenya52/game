# TODOS
- //TODO implement rezize_overworld_event, in game_movement
- //TODO give items in movement functions, line 80
- Rework materials
- implement buildable structures
- TODO find MAX u32 function for Water
- Write a good md
- Rework your game loop
- implement regrowable trees
- Idea for worldstates
// pub enum WORLD_STATE {
//   is_on_overworld,
//   is_in_cave,
//   is_in_a_fight
// }

-----------

# Dev sitenodes
## Predefined sets
### Blocktype
!!! Important !!! if the val of BlockType.durability is 404 it means the http code so there is no durability
- Index 0 is unbreakable -> You wont get anything from there, unpassable, like borders or portals/cave_entrance
- Index 1 is unbreakable_recources -> Is intended to set a drop later, because it has a durability 
- Index 2 is breakable_resource -> Is intended to set a drop later, also should disapear
- Index 3 is water -> Yea...
- Index 4 is air -> Uncappable block to push around
### Block
- Index 0 is stone
- Index 1 is dirt
- Index 2 is water
- Index 3 is wood
- Index 4 is food
- Index 5 is air
- Index 6 is Borders
- Index 7 is Cave entrance
- Index 8 is Minion
- Index 9 is Leaf
### Materials
- Index 0 is nothing
- Index 1 is wood
- Index 2 is stone
- Index 3 is dirt
- Index 4 is water
- Index 5 is food

# Events
## Movement
### rezize_overworld_event
- Double the size of the world every x turns as reference
- Needs the world as a muturable reference to resize it.
- Furthermore the function needs a number as orientation in the params
