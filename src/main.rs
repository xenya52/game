mod utils;
mod gameplay;
use gameplay::{get_user_input,handle_input, rezize_overworld_event};
mod world;
use world::{init_cave, init_overworld, World};
mod game_logic;
use game_logic::{BasicNeeds, Entity, game_over,
    show_entity_status, entity_moved, print_given_board};
fn main() {
    let overworld = init_overworld(32,16);
    let cave = init_cave(32,16);
    let mut world: World = World::new(overworld, cave);
    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    let mut debug_minion: Entity = Entity::new("minion_debug".to_string(),5, 1, 0, starter_needs, 20);
    let mut usr_input:char = 'x';

    while !game_over(usr_input, debug_minion.clone()) {
        rezize_overworld_event(&mut world, 5, debug_minion.turns);
        if world.is_on_overworld {
            print_given_board(&mut world.overworld);
            show_entity_status(&debug_minion);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world);
            entity_moved(&mut debug_minion);
        }
        else {
            print_given_board(&mut world.cave);
            show_entity_status(&debug_minion);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world);
            entity_moved(&mut debug_minion);
        }
    }
}
