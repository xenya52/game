use crate::world::Board;
//////////////////////
///External imports///
//////////////////////
use crossterm::style::{style, Stylize};

use super::Entity;

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
pub fn display_entity_inventory(entity: &mut Entity) {
    println!("<-=-=-=-=-=-=-=->");
    for (index, item) in entity.inventory.materials.iter_mut().enumerate() {
        if item.name != "nothing" {
            println!("<~~~~~~~~~~~~~~~>");
            print!("<{}. {} Amount = {}", index, item.name, item.amount);
            if index % 3 == 0 {
                print!("\n")
            }
            else {
                print!(" | ")
            }
        }
    }
    println!("<-=-=-=-=-=-=-=->");
}