use std::io;

use crate::world::Board;
use crate::game_logic::Player;
//////////////////////
///External imports///
//////////////////////
use crossterm::style::{style, Stylize};

/////////////////
//Print a board//
/////////////////
pub fn print_given_board(given_board: &mut Board, player: &mut Player) {
    for (_y, row) in given_board.iter_mut().enumerate() {
        for (_x, col) in row.iter_mut().enumerate() {
            let styled_content = style(col.display_ascii)
                .with(col.display_color);
            if player.y == _y && player.x == _x {
              print!("{}", styled_content.on_yellow())
            }
            else {
              print!("{}", styled_content);
            }
        }
        println!();
    }
}
pub fn print_keybindings() {
    println!("Movement [w] [a] [s] [d]");
    println!("Inventory [i]");
}
