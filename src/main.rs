use game::{
            init_overworld, 
            game_over, 
            print_overworld, 
            get_user_input, 
            handle_input, 
            move_preditor,
            Eniemy,
            Entity, 
            BasicNeeds,
            Materials,  
            show_entity_status,
            entity_moved,
            is_on_overworld,
            World,
            init_cave
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
            handle_input(usr_input, &mut world.overworld, &mut player);
            entity_moved(&mut player);
            move_preditor(&mut world.overworld, &mut eniemy);
        }
        else {
            println!("Underworld")
        }
    }
    println!("Hello, world!");
}
