use game::{init_board, game_over, print_board, get_user_input, handle_input, move_preditor};
fn main() {
    let mut board = init_board();

    let mut usr_input:char = 'x';
    while  !game_over(usr_input) {
        print_board(&mut board);
        usr_input = get_user_input();
        handle_input(usr_input, &mut board);
        move_preditor(&mut board);
    }
    println!("Hello, world!");
}
