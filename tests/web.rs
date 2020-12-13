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

#[wasm_bindgen_test]
pub fn test_count_0_neighbors() {
    let input_grid = Grid::create(3, 3, &[(1,1)]);
    let count = input_grid.live_neighbor_count(1, 1);
    assert_eq!(count, 0);
}
#[wasm_bindgen_test]
pub fn test_count_1_neighbor() {
    let input_grid = Grid::create(3, 3, &[(0,0)]);
    let count = input_grid.live_neighbor_count(1, 1);
    assert_eq!(count, 1);
}
#[wasm_bindgen_test]
pub fn test_count_2_neighbors() {
    let input_grid = Grid::create(3, 3, &[(0,1), (2,1)]);
    let count = input_grid.live_neighbor_count(1, 1);
    assert_eq!(count, 2);
}
#[wasm_bindgen_test]
pub fn test_count_8_neighbors() {
    let input_grid = Grid::create(3, 3, &[(0,0), (0,1),(0,2),(1,0),(1,2),(2,0),(2,1),(2,2)]);
    let count = input_grid.live_neighbor_count(1, 1);
    assert_eq!(count, 8);
}
#[wasm_bindgen_test]
pub fn test_count_2_edge_neighbors() {
    let input_grid = Grid::create(3, 3, &[(0,1),(1,1), (2,1)]);
    let count = input_grid.live_neighbor_count(0, 1);
    assert_eq!(count, 2);
}


// test the 3x3 blinker oscillator
//
// ◻◼◻      ◻◻◻
// ◻◼◻  ->  ◼◼◼
// ◻◼◻      ◻◻◻
//
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

// test the 3x3 blinker oscillator
//
// ◻◻◻      ◻◼◻
// ◼◼◼  ->  ◻◼◻
// ◻◻◻      ◻◼◻
//
#[wasm_bindgen_test]
pub fn test_blinker_2() {
    // Let's create a grid in known state
    let mut input_grid = Grid::create(5, 5, &[(2,1),(2,2),(2,3)]);

    // expected state after one generation
    let expected_grid = Grid::create(5, 5, &[(1,2),(2,2),(3,2)]);

    // advance the clock one generation
    input_grid.tick();

    // verify the state
    assert_eq!(&input_grid.get_cells(), &expected_grid.get_cells());
}

// test the beehive still life (doesn't change)
//
// ◻◼◼◻      ◻◼◼◻
// ◼◻◻◼  ->  ◼◻◻◼
// ◻◼◼◻      ◻◼◼◻
//
#[wasm_bindgen_test]
pub fn test_beehive() {
    // Let's create a grid in known state
    let mut input_grid = Grid::create(6, 5, &[(2,2),(2,3),(3,1),(3,4),(4,2),(4,3)]);

    // expected state after one generation
    let expected_grid = Grid::create(6, 5, &[(2,2),(2,3),(3,1),(3,4),(4,2),(4,3)]);

    // advance the clock one generation
    input_grid.tick();

    // verify the state
    assert_eq!(&input_grid.get_cells(), &expected_grid.get_cells());
}
// test the 4x8 from JP's example
// test a c/4 glider spaceship
