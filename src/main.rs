use game::{
            init_board, 
            game_over, 
            print_board, 
            get_user_input, 
            handle_input, 
            move_preditor
        };
//Testing functions
use game::{Entity, new_entity, show_entity_status};

fn main() {
    let mut board = init_board();
    let mut player: Entity = new_entity(5, 1, 10, 10, 10);
    let mut usr_input:char = 'x';
    while  !game_over(usr_input, player) {
        print_board(&mut board);
        show_entity_status(player);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board);
        move_preditor(&mut board);
    }
    println!("Hello, world!");
}
