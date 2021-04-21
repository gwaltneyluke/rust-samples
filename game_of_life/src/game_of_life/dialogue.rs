#![cfg_attr(feature = "strict", deny(warnings))]

use std::io::*;

pub struct MapInput {
  pub height: u8,
  pub width: u8,
  pub map_input_string: String
}

pub fn get_map_input() -> MapInput {
  let mut height_width_string = String::new();
  let mut map_input_string = String::new();
  print_prompt();

  stdin().read_line(&mut height_width_string).unwrap();
  height_width_string = height_width_string.trim_end().to_string();
  let (height, width) = get_height_width(&mut height_width_string);

  for _x in 0..height {
    stdin().read_line(&mut map_input_string).unwrap();
    map_input_string = map_input_string.trim_end().to_string();
  }

  MapInput { height, width, map_input_string }
}

fn print_prompt() {
  println!("Enter your civilization below with the given format:");
  println!("{{height}} {{width}}");
  println!("{{civilization grid}}");
  println!("Where the civilization grid marks dead cells with a period (.)");
  println!("and living cells with a period (*)");
  println!("");
  println!("For example:");
  println!("");
  println!("5 8");
  println!("........");
  println!("...**...");
  println!(".*****..");
  println!("........");
  println!("........");
  println!("");
  println!("");
  println!("Entry:");
  println!("");
}

fn get_height_width(input_string: &mut String) -> (u8, u8) {
  let height_width_vec = input_string.split(" ").collect::<Vec<&str>>();
  let height: u8 = height_width_vec[0].parse().unwrap();
  let width: u8 = height_width_vec[1].parse().unwrap();
  (height, width)
}