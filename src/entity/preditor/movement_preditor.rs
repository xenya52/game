pub fn move_preditor(board: &mut Board) {
    let possible_move: bool = rand::thread_rng().gen_bool(0.5);
    println!("{}",possible_move);
    if possible_move
     {
        let co_preditor: Vec<(u32,u32)> = find_char_in_board(board, 'ö');
        let co_player: Vec<(u32,u32)> = find_char_in_board(board, '@');
        // Assuming both vectors are guaranteed to have at least one element.
        // You should add checks to ensure they are not empty to avoid runtime panics.
        let resultx: i32 = co_preditor[0].0 as i32 - co_player[0].0 as i32; // Subtract x coordinates
        let resulty: i32 = co_preditor[0].1 as i32 - co_player[0].1 as i32; // Subtract y coordinates
        if resultx > 0 {
            if moveable(board, 'a', &co_preditor) {
                move_left(&co_preditor, board);
            }
            else{
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resultx < 0 {
            if moveable(board, 'd', &co_preditor) {
                move_right(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resulty < 0 {
            if moveable(board, 's', &co_preditor) {
                move_down(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
        else if resulty > 0 {
            if moveable(board, 'w', &co_preditor) {
                move_up(&co_preditor, board);
            }
            else {
                try_move_alternative(board, &co_preditor);
            }
        }
     }
}
fn try_alternative_preditor(board: &mut Board, co_preditor: &Vec<(u32, u32)>) {
    let directions = vec!['w', 'a', 's', 'd'];

    for &direction in &directions {
        match direction {
            'w' if moveable(board, 'w', co_preditor) => {
                move_up(co_preditor, board);
                break; 
            },
            'a' if moveable(board, 'a', co_preditor) => {
                move_left(co_preditor, board);
                break;
            },
            's' if moveable(board, 's', co_preditor) => {
                move_down(co_preditor, board);
                break;
            },
            'd' if moveable(board, 'd', co_preditor) => {
                move_right(co_preditor, board);
                break;
            },
            _ => continue,
        }
    }
}