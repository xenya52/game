use game::{
            init_board, 
            game_over, 
            print_overworld, 
            get_user_input, 
            handle_input, 
            move_preditor,
            EniemyEntity
        };
//Testing functions
use game::{Entity, BasicNeeds, Materials, show_entity_status, entity_moved};

fn main() {
    let mut board = init_board();
    let starter_needs: BasicNeeds = BasicNeeds::new(10, 10, 10);
    let starter_materials: Materials = Materials::new(0, 0);
    let mut player: Entity = Entity::new(5, 1, 0, starter_needs, starter_materials);
    let mut test: EniemyEntity = EniemyEntity::new(5, 1);
    let mut usr_input:char = 'x';
    while  !game_over(usr_input, player) {
        print_overworld(&mut board);
        show_entity_status(&player);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board, &mut player);
        entity_moved(&mut player);
        move_preditor(&mut board, &mut test);
    }
    println!("Hello, world!");
}
