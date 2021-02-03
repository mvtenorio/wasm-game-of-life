mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    height: u32,
    width: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    /// Creates a universe with the provided `width` and `height`.
    pub fn new(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, Math::random() > 0.5)
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    /// Get the width of the universe.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe.
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells();
    }

    /// Get the height of the universe.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the height of the universe.
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells();
    }

    /// Get a pointer for the cells of the universe.
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Find the index for the cell given `row` and `col`
    #[wasm_bindgen(js_name = getIndex)]
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Produces the next state based on the current state,
    /// according to the following rules:
    ///
    /// - Rule 1: Any live cell with fewer than two live neighbours
    /// dies, as if caused by underpopulation.
    /// - Rule 2: Any live cell with two or three live neighbours
    /// lives on to the next generation.
    /// - Rule 3: Any live cell with more than three live
    /// neighbours dies, as if by overpopulation.
    /// - Rule 4: Any dead cell with exactly three live neighbours
    /// becomes a live cell, as if by reproduction.
    ///
    /// All other cells remain in the same state.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }
        self.cells = next;
    }

    /// Resets all cells to the dead state.
    pub fn reset_cells(&mut self) {
        let size = (self.width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells;
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

}
