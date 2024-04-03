use crate::generator::generator_tools::tools::find_possible_numbers;
use rand::Rng;

pub struct PuzzleGenerator<'a> {
    sudoku_fields: &'a Vec<Vec<u8>>,
    puzzle_fields: Vec<Vec<u8>>,
}

impl PuzzleGenerator<'_> {
    pub fn run(sudoku_fields: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut this = PuzzleGenerator {
            sudoku_fields,
            puzzle_fields: vec![vec![0; 9]; 9],
        };
        this.initial_puzzle_fields();
        while !Self::is_solvable(&this.puzzle_fields) {
            this.add_a_field();
        }
        this.remove_fields_as_much_as_possible();
        this.puzzle_fields
    }

    fn initial_puzzle_fields(&mut self) {
        for i in 0..9 {
            let mut rng = rand::thread_rng();
            for _ in 0..2 {
                let col = rng.gen_range(0..9);
                self.puzzle_fields[i][col] = self.sudoku_fields[i][col];
            }
        }
    }

    fn is_solvable(puzzle_fields: &[Vec<u8>]) -> bool {
        let mut puzzle = puzzle_fields.to_owned();

        let mut counter = puzzle.iter().flatten().filter(|&&x| x != 0).count();
        while counter < 81 {
            let mut is_changed = false;
            for row in 0..9 {
                for col in 0..9 {
                    if puzzle[row][col] == 0 {
                        let possible_numbers = find_possible_numbers(&puzzle, row, col);
                        if possible_numbers.len() == 1 {
                            puzzle[row][col] = possible_numbers[0];
                            is_changed = true;
                            counter += 1;
                        }
                    }
                }
            }
            if !is_changed {
                return false;
            }
        }
        true
    }

    fn add_a_field(&mut self) {
        let (field_row, field_col) = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .filter(|&(row, col)| self.puzzle_fields[row][col] == 0)
            .max_by_key(|&(row, col)| find_possible_numbers(&self.puzzle_fields, row, col).len())
            .unwrap_or((0, 0));

        self.puzzle_fields[field_row][field_col] = self.sudoku_fields[field_row][field_col];
    }

    fn remove_fields_as_much_as_possible(&mut self) {
        for (row, col) in (0..9).flat_map(|row| (0..9).map(move |col| (row, col))) {
            if self.puzzle_fields[row][col] == 0 {
                continue;
            }
            self.puzzle_fields[row][col] = 0;
            if !Self::is_solvable(&self.puzzle_fields) {
                self.puzzle_fields[row][col] = self.sudoku_fields[row][col];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models;

    #[test]
    fn test_puzzle_generator_difficulty() {
        let sudoku = models::sudoku::Sudoku::new();
        let puzzle = PuzzleGenerator::run(&sudoku.complete_sudoku);

        let visible_numbers_count = puzzle.iter().flatten().filter(|&&x| x != 0).count();
        assert!(
            visible_numbers_count < 35,
            "Generated puzzle is too easy. It has {} visible numbers, expected less than 35.",
            visible_numbers_count
        );
    }

    #[test]
    fn test_is_solvable() {
        let sudoku = models::sudoku::Sudoku::new();
        let mut puzzle = PuzzleGenerator::run(&sudoku.complete_sudoku);
        assert_eq!(PuzzleGenerator::is_solvable(&puzzle), true);
        for row in 0..9 {
            for col in 0..9 {
                if puzzle[row][col] != 0 {
                    puzzle[row][col] = 0;
                    assert_eq!(PuzzleGenerator::is_solvable(&puzzle), false);
                    puzzle[row][col] = sudoku.complete_sudoku[row][col];
                }
            }
        }
    }
}
