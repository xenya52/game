# TODOS
- implement "playercursor"
- implement buildable structures
- Let spawn caves randomly and make then unique
- implement render distance
- Rework your game loop / Watch space invaders tutorial
- implement regrowable trees
- Idea for worldstates
// pub enum WORLD_STATE {
//   is_on_overworld,
//   is_in_cave,
//   is_in_a_fight
// }
- TODO make items struct
// Existing item names (without a clear struct or something like that) 
// - Index 0 is nothing
// - Index 1 is wood
// - Index 2 is stone
// - Index 3 is dirt
// - Index 4 is water
// - Index 5 is food

-----------

# Dev sitenodes
## Enums
### display
!!! Important !!! Defines the current display state, as the name say
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
### Existing item names (without a clear struct or something like that)
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

# Structs
## Enitity
### Attributes
 - *name* It's self explaining
 - *health* The HP like from other videogames
 - *strengh* How much damage the entity can make
 - *actions* How mutch actions the entity made since it is alive
 - *basic_needs* See the struct "BasicNeeds"
 - *inventory* See the struct "Inventory"
### Functions
 - *new* -> Creates a new entity
 - *block_to_inventory* -> Adds a block into the existing inventory struct that's using the entity
 - *show_entity_status* -> Shows the name, health, strengh and basic_needs, that I mentioned earlyer, ascii printed in the console
 - *display_entity_inventory* -> Displays the inventory, owned by the given entity, ascii printed in the console
## BasicNeeds
!!! TODO !!!
## Inventory
### Attributes
- *items* A Vector of Strings
### Functions
- *new* -> Creates a new Inventory with empty tiles as "---"
- *add* -> Adds a String value to the *items* Vector
- *remove* -> Replaces a String value to the empty tiles "---", in the *items* Vector
## Player
 - Basic idea is that the player handle basic attributes that the game will need in much functions 
### Attributes 
 - *display_state* A Enum that holds the current printing state like "openend_inventory" etc...
 - *last_input* Is the last input that the user of the program made *TODO IMPLEMENT*
### Functions 
  - *new* -> Creates a bew Player object, the new function should also never take any parames because the start values are trivial
  - *change_displaying_state* -> The name is self explaining
