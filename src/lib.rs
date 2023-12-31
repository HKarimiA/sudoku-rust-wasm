mod generator;
mod models;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub value: u8,
}

#[wasm_bindgen]
pub struct Sudoku {
    sudoku: models::sudoku::Sudoku,
}

#[wasm_bindgen]
impl Sudoku {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sudoku {
        Sudoku {
            sudoku: models::sudoku::Sudoku::new(),
        }
    }

    pub fn get_puzzle(&self) -> Vec<u8> {
        self.sudoku
            .puzzle_sudoku
            .iter()
            .flatten()
            .map(|x| *x)
            .collect::<Vec<u8>>()
    }

    pub fn input(&mut self, row: usize, col: usize, value: u8) {
        self.sudoku.user_input(row, col, value);
    }

    pub fn has_won(&self) -> bool {
        self.sudoku.has_won()
    }

    pub fn incorrect_fields(&self) -> Vec<usize> {
        self.sudoku
            .incorrect_fields()
            .iter()
            .map(|&(row, col)| row * 9 + col)
            .collect::<Vec<usize>>()
    }

    pub fn next_step(&mut self) -> Option<Cell> {
        Some(
            self.sudoku
                .next_step()
                .map(|(row, col, value)| Cell { row, col, value })?,
        )
    }
}
