pub(crate) mod tools {
    pub(crate) fn find_possible_numbers(fields: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
        let mut possible_numbers: Vec<u8> = (1..=9).collect();

        for i in 0..9 {
            if fields[row][i] != 0 {
                if let Some(pos) = possible_numbers.iter().position(|&x| x == fields[row][i]) {
                    possible_numbers.remove(pos);
                }
            }
            if fields[i][col] != 0 {
                if let Some(pos) = possible_numbers.iter().position(|&x| x == fields[i][col]) {
                    possible_numbers.remove(pos);
                }
            }
        }

        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for i in start_row..start_row + 3 {
            for j in start_col..start_col + 3 {
                if fields[i][j] != 0 {
                    if let Some(pos) = possible_numbers.iter().position(|&x| x == fields[i][j]) {
                        possible_numbers.remove(pos);
                    }
                }
            }
        }

        possible_numbers
    }
}
