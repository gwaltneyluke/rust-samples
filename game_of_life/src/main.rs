#![cfg_attr(feature = "strict", deny(warnings))]

mod game_of_life;

use game_of_life::dialogue::{MapInput, get_map_input};
use game_of_life::navigation::{Map};
use game_of_life::translation::{map_input_to_map};

#[allow(dead_code)]
fn main() {
  println!("Welcome to the Game of Life!");
  println!("");

  let map_input: MapInput = get_map_input();
  let map: Map = map_input_to_map(map_input);
  
  println!("");
  println!("next generation:");
  println!("{}", map.get_next_gen());
}