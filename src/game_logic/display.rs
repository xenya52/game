use crate::world::Board;
//////////////////////
///External imports///
//////////////////////
use crossterm::style::{style, Stylize};
/////////////////
//Print a board//
/////////////////
pub fn print_given_board(given_board: &mut Board) {
    for (_y, row) in given_board.iter_mut().enumerate() {
        for (_x, col) in row.iter_mut().enumerate() {
            let styled_content = style(col.display_ascii)
                .with(col.display_color);
            print!("{}", styled_content);
        }
        println!();
    }
}
pub fn print_keybindings() {
    println!("Movement [w] [a] [s] [d]");
    println!("Inventory [i]");
}
