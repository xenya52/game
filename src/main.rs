mod utils;
mod gameplay;
use gameplay::{get_user_input,handle_input, rezize_overworld_event};
mod world;
use world::{init_cave, init_overworld, World};
mod game_logic;
use game_logic::{BasicNeeds, Entity, game_over, place_minion, 
    entity_moved, print_given_board, print_keybindings,
    Player, Displaying};
fn main() {
    let mut player = Player::new();
    let overworld = init_overworld(8,8);
    let cave = init_cave(32,16);
    let mut world: World = World::new(overworld, cave);
    place_minion(&mut world.overworld);

    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    let mut debug_minion: Entity = Entity::new("minion_debug".to_string(),5, 1, 0, starter_needs, 20);

    while !game_over(player.last_input, debug_minion.clone()) {
        if player.display_state == Displaying::Overworld {
            print_given_board(&mut world.overworld);
            print_keybindings();
            Entity::show_entity_status(&debug_minion);
            player.last_input = get_user_input();
            handle_input(&mut player, &mut world, &mut debug_minion);
            entity_moved(&mut debug_minion);
            rezize_overworld_event(&mut world, 25, debug_minion.actions as u32);
        }
        else if player.display_state == Displaying::Cave {
            print_given_board(&mut world.cave);
            Entity::show_entity_status(&debug_minion);
            player.last_input = get_user_input();
            handle_input(&mut player, &mut world, &mut debug_minion);
            entity_moved(&mut debug_minion);
        }
    }
}
