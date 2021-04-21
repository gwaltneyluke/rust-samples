#![cfg_attr(feature = "strict", deny(warnings))]

use super::dialogue::{MapInput};
use super::navigation::{Map, Player};

pub fn map_input_to_map(map_input: MapInput) -> Map {
  let MapInput { height, width, map_input_string } = map_input;
  let map_input_array = map_input_string.chars().collect::<Vec<char>>();
  let mut players = vec![vec![Player { is_alive: false }; width as usize]; height as usize];
  let lower_bound = height as usize;
  let right_bound = width as usize;

  for y in 0..lower_bound {
    for x in 0..right_bound {
      let input_index = ( y * right_bound ) + x;
      if map_input_array[input_index] == '*' {
        players[y][x].is_alive = true;
      }
    }
  }

  Map { height, width, players }
}