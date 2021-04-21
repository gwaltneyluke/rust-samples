#![cfg_attr(feature = "strict", deny(warnings))]

use game_of_life::translation::{MapInput, map_input_to};

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, game_of_life::answer());
}
