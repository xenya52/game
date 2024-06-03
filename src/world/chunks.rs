use crate::world::{
    Board,
    Block,
    add_radom_food, add_radom_water, add_random_mountain, 
    add_cave, add_border, add_cave_exit};
use crate::game_logic::place_minion;

pub fn init_overworld(x_size: usize, y_size: usize) -> Board {
  print!("x = {}", x_size);
  print!("y = {}", y_size);
  let mut board: Vec<Vec<Block>> = vec![
                                   vec![Block::new_predefined_set()[5].clone();
                                  x_size];y_size];
  let mut counter = 0;

  //Generate barriers
  print!("Generate mountians ... ");
  loop {
      counter += 1;
      add_random_mountain(&mut board);
      if counter == 10 {
          break;
      }
  }
  counter = 0;
  println!("Done!");
  //Generate water
  print!("Generate water ... ");
  loop {
      counter += 1;
      add_radom_water(&mut board);
      if counter == 3 {
          break;
      }
  }
  counter = 0;
  println!("Done!");
  //Generate food
  print!("Generate food ... ");
  loop {
      counter += 1;
      add_radom_food(&mut board);
      if counter == 4 {
          break;
      }
  }
  println!("Done!");
  //Generate cave entrance
  print!("Generate cave entrance ...");
  add_cave(&mut board);
  println!("Done!");
  //Set player in board
  print!("Set one minion (Debug) in board ... ");
  place_minion(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  add_border(&mut board);
  println!("Done!");
  return board;
}

pub fn init_cave(x_size: usize, y_size: usize) -> Board {
    let mut board: Vec<Vec<Block>> = vec![vec![
        Block::new_predefined_set()[5].clone();
        x_size];y_size];
  let mut count = 0;
  //Generate barriers
  print!("Generate rock ... ");
  loop {
      count += 1;
      add_random_mountain(&mut board);
      if count == 10 {
          break;
      }
  }
  count = 0;
  println!("Done!");
  //Generate water
  print!("Generate water ... ");
  loop {
      count += 1;
      add_radom_water(&mut board);
      if count == 3 {
          break;
      }
  }
  println!("Done!");
  //Set player in cave
  print!("Set one minion (debug) in board ... ");
  place_minion(&mut board);
  println!("Done!");
  //Set cave exit
  print!("Generate cave exit ...");
  add_cave_exit(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  add_border(&mut board);
  println!("Done!");
  return board;
}
