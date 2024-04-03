use crate::generator::generator_tools::tools::find_possible_numbers;
use crate::generator::puzzle_generator::PuzzleGenerator;
use crate::generator::sudoku_generator::SudokuGenerator;

pub struct Sudoku {
    pub complete_sudoku: Vec<Vec<u8>>,
    pub puzzle_sudoku: Vec<Vec<u8>>,
    pub display_sudoku: Vec<Vec<u8>>,
}

impl Sudoku {
    pub fn new() -> Self {
        let sudoku = SudokuGenerator::run();
        let puzzle = PuzzleGenerator::run(&sudoku);
        Self {
            complete_sudoku: sudoku,
            puzzle_sudoku: puzzle.clone(),
            display_sudoku: puzzle,
        }
    }

    pub fn user_input(&mut self, row: usize, col: usize, value: u8) {
        self.display_sudoku[row][col] = value;
    }

    pub fn has_won(&self) -> bool {
        self.display_sudoku == self.complete_sudoku
    }

    pub fn incorrect_fields(&self) -> Vec<(usize, usize)> {
        (0..9)
            .flat_map(|row| {
                (0..9)
                    .filter(move |&col| {
                        self.display_sudoku[row][col] != self.complete_sudoku[row][col]
                            && self.display_sudoku[row][col] != 0
                    })
                    .map(move |col| (row, col))
            })
            .collect()
    }

    pub fn next_step(&mut self) -> Option<(usize, usize, u8)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.display_sudoku[row][col] == 0 {
                    let possible_numbers = find_possible_numbers(&self.display_sudoku, row, col);
                    if possible_numbers.len() == 1
                        && possible_numbers[0] == self.complete_sudoku[row][col]
                    {
                        self.display_sudoku[row][col] = possible_numbers[0];
                        return Some((row, col, possible_numbers[0]));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_user_input() {
        let mut sudoku = Sudoku::new();
        sudoku.user_input(0, 0, 1);
        assert_eq!(sudoku.display_sudoku[0][0], 1);
    }

    #[test]
    fn test_sudoku_has_won() {
        let mut sudoku = Sudoku::new();
        assert_eq!(sudoku.has_won(), false);
        sudoku.display_sudoku = sudoku.complete_sudoku.clone();
        assert_eq!(sudoku.has_won(), true);
    }

    #[test]
    fn test_sudoku_incorrect_fields() {
        let mut sudoku = Sudoku::new();
        assert_eq!(sudoku.incorrect_fields(), Vec::<(usize, usize)>::new());
        sudoku.display_sudoku[0][0] = sudoku.complete_sudoku[0][0] + 1;
        assert_eq!(sudoku.incorrect_fields(), vec![(0, 0)]);
    }
}
