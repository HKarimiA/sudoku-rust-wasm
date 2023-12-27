use crate::generator::generator::tools::find_possible_numbers;
use rand::Rng;

pub struct SudokuGenerator {
    fields: Vec<Vec<u8>>,
    filled_fields: u8,
    reset_counter: u8,
}

impl SudokuGenerator {
    pub fn run() -> Vec<Vec<u8>> {
        let fields = vec![vec![0; 9]; 9];
        let mut this = SudokuGenerator {
            fields,
            filled_fields: 0,
            reset_counter: 0,
        };
        this.fill_all_fields();
        this.fields
    }

    fn fill_zeros(&mut self) {
        self.fields
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|col| *col = 0));
        self.filled_fields = 0;
        self.reset_counter = 0;
    }

    fn fill_all_fields(&mut self) {
        while self.filled_fields < 81 {
            let (row, col, number) = self.find_first_field_to_fill();
            self.fields[row][col] = number;
            self.filled_fields = self.filled_fields + 1;
        }
    }

    fn find_first_field_to_fill(&mut self) -> (usize, usize, u8) {
        let mut ret_row = 0;
        let mut ret_col = 0;
        let mut ret_possible_nr: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for row in 0..9 {
            for col in 0..9 {
                if self.fields[row][col] != 0 {
                    continue;
                }
                let possible_numbers = find_possible_numbers(&self.fields, row, col);
                if possible_numbers.is_empty() {
                    self.reset_all_relative_fields(row, col);
                    return self.find_first_field_to_fill();
                }
                if possible_numbers.len() < ret_possible_nr.len() {
                    ret_row = row;
                    ret_col = col;
                    ret_possible_nr = possible_numbers;
                }
            }
        }
        let ret_number = ret_possible_nr[rand::thread_rng().gen_range(0..ret_possible_nr.len())];
        (ret_row, ret_col, ret_number)
    }

    fn reset_all_relative_fields(&mut self, row: usize, col: usize) {
        if self.reset_counter > 4 {
            self.fill_zeros();
            return;
        }
        let mut reset_field = |r: usize, c: usize| {
            if self.fields[r][c] != 0 {
                self.fields[r][c] = 0;
                self.filled_fields -= 1;
            }
        };
        for i in 0..9 {
            reset_field(row, i);
            reset_field(i, col);
        }
        let block_start_row = (row / 3) * 3;
        let block_start_col = (col / 3) * 3;
        for i in block_start_row..block_start_row + 3 {
            for j in block_start_col..block_start_col + 3 {
                reset_field(i, j);
            }
        }
        self.reset_counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generated_sudoku_all_filled() {
        let sudoku = SudokuGenerator::run();
        for row in 0..9 {
            for col in 0..9 {
                assert_ne!(sudoku[row][col], 0);
            }
        }
    }

    #[test]
    fn test_generated_sudoku_is_valid() {
        let sudoku = SudokuGenerator::run();
        for row in 0..9 {
            for col in 0..9 {
                assert_ne!(sudoku[row][col], 0);
                let number = sudoku[row][col];
                for i in 0..9 {
                    if i != col {
                        assert_ne!(sudoku[row][i], number);
                    }
                    if i != row {
                        assert_ne!(sudoku[i][col], number);
                    }
                    for i in 0..3 {
                        for j in 0..3 {
                            if row / 3 * 3 + i != row && col / 3 * 3 + j != col {
                                assert_ne!(sudoku[row / 3 * 3 + i][col / 3 * 3 + j], number);
                            }
                        }
                    }
                }
            }
        }
    }
}
