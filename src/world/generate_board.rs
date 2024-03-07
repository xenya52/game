use rand::{thread_rng, Rng, seq::SliceRandom};


pub type Board = Vec<Vec<Title>>;
type Title = char;

pub fn init_board() -> Board {
    let mut board = vec![vec!['#';16];16];
    let mut count = 0;
    loop {
        count += 1;
        add_random_tile(&mut board);
        if count == 10 {
            break;
        }
    }
    set_player_in_board(&mut board);
    set_preditor_in_board(&mut board);
    return board;
}
fn add_random_tile(board: &mut Board) {
    let mut empty_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == '#' {
                empty_tiles.push((i, j));
            }
        }
    }

    if let Some(&(x, y)) = empty_tiles.choose(&mut thread_rng()) {
        let value: char = if thread_rng().gen_bool(0.9) { 'x' } else { 'X' };
        board[x][y] = value;
    }
}