pub fn init_overworld() -> Board {
  let mut board = vec![vec!['#';32];16];
  let mut count = 0;
  //Generate barriers
  print!("Generate mountians ... ");
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
  count = 0;
  println!("Done!");
  //Generate food
  print!("Generate food ... ");
  loop {
      count += 1;
      add_radom_food(&mut board);
      if count == 4 {
          break;
      }
  }
  println!("Done!");
  //Generate cave entrance
  print!("Generate cave entrance ...");
  add_cave(&mut board);
  println!("Done!");
  //Set preditor in board
  print!("Set preditor in board ... ");
  place_eniemy(&mut board);
  println!("Done!");
  //Set player in board
  print!("Set player in board ... ");
  place_player(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  set_frame_in_board(&mut board);
  println!("Done!");
  return board;
}
pub fn init_cave() -> Board {
  let mut board = vec![vec!['#';32];16];
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
  print!("Set player in board ... ");
  place_player(&mut board);
  println!("Done!");
  //Set frame
  print!("Set frame ... ");
  set_frame_in_board(&mut board);
  println!("Done!");
  return board;
}