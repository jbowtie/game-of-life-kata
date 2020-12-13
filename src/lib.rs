mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, game-of-life-kata!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

//functions in this section are Rust-only
impl Grid {
    // used by test
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // given (row, col) get cell index
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn set_live_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
    
    // create grid with given width, height, and live cells
    pub fn create(width: u32, height: u32, live: &[(u32, u32)]) -> Grid {
        // init cells as Dead
        let cells = (0..width * height)
            .map(|_i| { Cell::Dead })
            .collect();

        let mut grid = Grid {
            width,
            height,
            cells,
        };
        grid.set_live_cells(live);
        grid
    }
}

// functions in this section are exposed to browser
#[wasm_bindgen]
impl Grid {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        // TODO: actual changes here
        self.cells = next;
    }
}
