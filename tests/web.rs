//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate game_of_life_kata;
use game_of_life_kata::Grid;

#[wasm_bindgen_test]
pub fn test_empty_grid_creation() {
    // Let's create a grid in known state
    let input_grid = Grid::create(2, 2, &[]);
    let expected = "◻◻\n\
                    ◻◻\n";

    // verify the state
    assert_eq!(input_grid.to_string(), expected);
}

#[wasm_bindgen_test]
pub fn test_live_grid_creation() {
    // This tests we are creating the grid in the expected state
    let input_grid = Grid::create(3, 3, &[(0,1), (1,1), (2,1)]);
    let expected = "◻◼◻\n\
                    ◻◼◻\n\
                    ◻◼◻\n";

    // verify the state
    assert_eq!(input_grid.to_string(), expected);
}


// test the 3x3 blinker oscillator
#[wasm_bindgen_test]
pub fn test_blinker_1() {
    // Let's create a grid in known state
    let mut input_grid = Grid::create(5, 5, &[(1,2),(2,2),(3,2)]);

    // expected state after one generation
    let expected_grid = Grid::create(5, 5, &[(2,1),(2,2),(2,3)]);

    // advance the clock one generation
    input_grid.tick();

    // verify the state
    assert_eq!(&input_grid.get_cells(), &expected_grid.get_cells());
}

// test the 4x8 from JP's example
// test the 4x3 beehive still life
// test a c/4 glider spaceship
