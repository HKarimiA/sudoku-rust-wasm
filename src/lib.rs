mod generator;
mod models;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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

    pub fn get_puzzle(&self) -> Box<[JsValue]> {
        self.sudoku
            .puzzle_sudoku
            .iter()
            .flatten()
            .map(|x| x.to_string().into())
            .collect::<Vec<JsValue>>()
            .into_boxed_slice()
    }

    pub fn input(&mut self, row: usize, col: usize, value: u8) {
        self.sudoku.user_input(row, col, value);
    }

    pub fn has_won(&self) -> bool {
        self.sudoku.has_won()
    }

    pub fn incorrect_fields(&self) -> Box<[JsValue]> {
        self.sudoku
            .incorrect_fields()
            .iter()
            .map(|&(row, col)| {
                let index = row * 9 + col;
                index.to_string().into()
            })
            .collect::<Vec<JsValue>>()
            .into_boxed_slice()
    }

    pub fn next_step(&mut self) -> Option<Cell> {
        Some(
            self.sudoku
                .next_step()
                .map(|(row, col, value)| Cell { row, col, value })?,
        )
    }
}
