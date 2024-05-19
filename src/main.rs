use game::{
            entity_moved, game_over, get_user_input, handle_input, init_cave, init_overworld, change_world_state, move_preditor, print_cave, print_overworld, show_entity_status, BasicNeeds, Eniemy, Entity, Materials, World
        };

fn main() {
    let mut overworld = init_overworld();
    let mut cave = init_cave();
    let mut world: World = World::new(overworld, cave);
    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    let starter_materials: Materials = Materials::new(0, 0);
    let mut player: Entity = Entity::new(5, 1, 0, starter_needs, starter_materials);
    let mut eniemy: Eniemy = Eniemy::new(5, 1);

    let mut usr_input:char = 'x';

    while  !game_over(usr_input, player) {
        if world.is_on_overworld {
            print_overworld(&mut world.overworld);
            show_entity_status(&player);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world, &mut player);
            entity_moved(&mut player);
            move_preditor(&mut world.overworld, &mut eniemy);
        }
        else {
            print_cave(&mut world.cave);
            usr_input = get_user_input();
            handle_input(usr_input, &mut world, &mut player);
            entity_moved(&mut player);
            move_preditor(&mut world.cave, &mut eniemy);
        }
    }
    println!("Hello, world!");
}
