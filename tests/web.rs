//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
/*
// test the 3x3 blinker oscillator
#[wasm_bindgen_test]
pub fn test_blinker_1() {
    // Let's create a grid in known state
    let mut input_grid = input_state();

    // expected state after one generation
    let expected_grid = expected_state();

    // advance the clock one generation
    input_grid.tick();

    // verify the state
    assert_eq!(&input_grid.get_cells(), &expected_grid.get_cells());
}
*/
// test the 4x8 from JP's example
// test the 4x3 beehive still life
// test a c/4 glider spaceship
