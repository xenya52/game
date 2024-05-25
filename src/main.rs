mod utils;
mod gameplay;
use gameplay::{get_user_input,handle_input};
mod world;
use world::{World, init_cave, init_overworld};
mod game_logic;
use game_logic::{BasicNeeds, Materials, Entity, game_over, 
    print_overworld, print_cave,
    show_entity_status, entity_moved};
fn main() {
    let mut overworld = init_overworld();
    let mut cave = init_cave();
    let mut world: World = World::new(overworld, cave);
    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    let starter_materials: Materials = Materials::new(0, 0);
    let mut debug_minion: Entity = Entity::new(5, 1, 0, starter_needs, starter_materials);
    let mut usr_input:char = 'x';

    while !game_over(usr_input, debug_minion) {
        if world.is_on_overworld {
            print_overworld(&mut world.overworld);
            show_entity_status(&debug_minion);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world, &mut debug_minion);
            entity_moved(&mut debug_minion);
        }
        else {
            print_cave(&mut world.cave);
            show_entity_status(&debug_minion);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world, &mut debug_minion);
            entity_moved(&mut debug_minion);
        }
    }
    println!("Hello, world!");
}
