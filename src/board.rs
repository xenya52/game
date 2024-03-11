pub use rand::{thread_rng, Rng, seq::SliceRandom};
pub use colorized::*;
//Own stuff lib.rs
use crate::get_rdm_xy;

/////////
//Board//
/////////
pub type Board = Vec<Vec<Title>>;
type Title = char;

////////////////////
//World generation//
////////////////////
fn add_radom_barrier(board: &mut Board) {
    let value: char = if thread_rng().gen_bool(0.9) { 'x' } else { 'X' };
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = value;
}
fn set_player_in_board(board: &mut Board) {
    board[8][8] = '@';
}
fn set_preditor_in_board(board: &mut Board) {
    let c:Vec<usize> = get_rdm_xy(board);
    board[c[0]][c[1]] = 'ö';
}
pub fn init_board() -> Board {
    let mut board = vec![vec!['#';16];16];
    let mut count = 0;
    loop {
        count += 1;
        add_radom_barrier(&mut board);
        if count == 10 {
            break;
        }
    }
    set_player_in_board(&mut board);
    set_preditor_in_board(&mut board);
    return board;
}
////////////////
//Show a board//
////////////////
pub fn print_board(board: &mut Board) {
    for (i, row) in board.iter().enumerate() {
        for (j,row) in row.iter().enumerate() {
            if row == &'@' {
                print!("{}",Colors::BrightYellowFg.value());
            }
            else if row == &'ö' {
                print!("{}",Colors::RedFg.value())
            }
            else if row == &'x' || row == &'X' {
                print!("{}",Colors::BlackFg.value());
            }
            else {
                print!("{}",Colors::GreenFg.value());
            }
            print!("{}", row);
            print!("{}",Colors::Reset.value());
        }
        println!();
    }
}