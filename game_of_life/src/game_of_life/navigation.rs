#![cfg_attr(feature = "strict", deny(warnings))]

use std::fmt::*;
use std::marker::*;

pub struct Map {
  pub height: u8,
  pub width: u8,
  pub players: Vec<Vec<Player>>
}

impl Display for Map {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for row in &self.players {
      for player in row {
        write!(f, "{}", player);
      }
      write!(f, "\n");
    }
    write!(f, "")
  }
}

impl Map {
  pub fn get_next_gen(&self) -> Map {
    let mut next_gen_players = vec![vec![Player { is_alive: false }; self.width as usize]; self.height as usize];
    for y in 0usize..self.height as usize {
      for x in 0usize..self.width as usize {
        next_gen_players[y][x].is_alive = get_next_gen_is_alive(self, x, y);
      }
    }
    Map {
      height: self.height,
      width: self.width,
      players: next_gen_players
    }
  }
}

fn get_next_gen_is_alive(map: &Map, x: usize, y: usize) -> bool {
  let mut living_neighbors_count = 0u8;
  let lower_bound = map.height as i8;
  let right_bound = map.width as i8;

  for y_offset in -1i8..=1 {
    for x_offset in -1i8..=1 {
      if y_offset != 0 || x_offset != 0 {
        let (in_bounds, (x_index, y_index)) = get_indeces(x, x_offset, y, y_offset, right_bound, lower_bound);
        if in_bounds && map.players[y_index][x_index].is_alive {
          living_neighbors_count += 1;
        }
      }
    }
  }

  match living_neighbors_count {
    2 => map.players[y][x].is_alive,
    3 => true,
    _ => false
  }
}

fn get_indeces(x: usize, x_off: i8, y: usize, y_off: i8, x_max: i8, y_max: i8) -> (bool, (usize, usize)) {
  let x_in_bounds = x as i8 + x_off;
  let y_in_bounds = y as i8 + y_off;

  if x_in_bounds > 0 && x_in_bounds < x_max && y_in_bounds > 0 && y_in_bounds < y_max {
    (true, (x_in_bounds as usize, y_in_bounds as usize))
  } else {
    (false, (0usize, 0usize))
  }

}

#[derive(Copy, Clone)]
pub struct Player {
  pub is_alive: bool
}

impl Display for Player {
  fn fmt(&self, f: &mut Formatter) -> Result {
    if self.is_alive { write!(f, "*") }
    else { write!(f, ".") }
  }
}