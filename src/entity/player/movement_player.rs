
pub fn handle_input(usr_input: char, board: &mut Board) {
    let coordinates: Vec<(u32, u32)> = find_char_in_board(board, '@');
    if moveable(board, usr_input, &coordinates) {
        match usr_input {
            'w' => move_up(&coordinates ,board),
            'a' => move_left(&coordinates, board),
            's' => move_down(&coordinates, board),
            'd' => move_right(&coordinates, board),
            _ => println!("Error")
        }
    }
}
pub fn get_user_input() -> char {
    let mut input = input();
    match input.read_char() {
        Ok(c) => c,
        Err(e) => {
            'e'
        }
    }   
}