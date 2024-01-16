pub use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, MouseButton, MouseEvent, SyncReader, TerminalInput};
pub use colorized::*;

type Title = char;
type Board = Vec<Vec<Title>>;

enum UserAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    None,
    Quit
}

fn get_user_input() {
    let mut input = input();
    match input.read_char() {
        Ok(s) => println!("char typed: {}", s),
        Err(e) => println!("char error : {}", e),
     }
}

fn init_board() -> Board {
    vec![vec!['#';16];16]
}

fn set_player_in_board(board: &mut Board){

    board[8][8] = '@';
}

fn print_board(board: &mut Board) {
    for (i, row) in board.iter().enumerate() {
        for (j,row) in row.iter().enumerate() {
            if row == &'@' {
                print!("{}",Colors::MagentaFg.value());
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

fn main() {
    let mut board = init_board();
    set_player_in_board(&mut board);
    print_board(&mut board);
    println!("Hello, world!");
}
