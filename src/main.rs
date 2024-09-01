mod utils;
mod gameplay;
use gameplay::{ get_user_input,handle_input };
mod world;
use utils::find_char_in_board;
use world::{init_cave, init_overworld, World};
mod game_logic; 
use game_logic::{BasicNeeds, Entity, game_over, place_minion_ascii, 
    entity_moved, print_given_board, print_keybindings,
    Player, Displaying};
fn main() {
    let overworld = init_overworld(500,500);
    let cave = init_cave(32,16);
    let mut world: World = World::new(overworld, cave);
    
    let mut player = Player::new(3, 3, 8);
    
    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    place_minion_ascii(&mut world.overworld);
    let entity_coords = find_char_in_board(&world.overworld, '@');
    let mut debug_minion: Entity = Entity::new(entity_coords[1], entity_coords[0],"minion_debug".to_string(),5, 1, 0, starter_needs, 20);
    
    while !game_over(player.last_input, debug_minion.clone()) {
        if player.display_state == Displaying::Overworld {
          print_given_board(&mut world.overworld, &mut player);
          Entity::show_entity_status(&debug_minion);
        }
        else if player.display_state == Displaying::Cave {
          print_given_board(&mut world.cave, &mut player);
          Entity::show_entity_status(&debug_minion);
        }
        else if player.display_state == Displaying::Inventory {
          Entity::display_entity_inventory(&mut debug_minion);
        }
        player.last_input = get_user_input();
        handle_input(&mut player, &mut world, &mut debug_minion);
        if player.display_state != Displaying::Inventory {
          entity_moved(&mut world, &mut debug_minion, &mut player);
        }
        print_keybindings();
    }
}
