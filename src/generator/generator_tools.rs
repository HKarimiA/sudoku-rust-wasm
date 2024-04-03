pub(crate) mod tools {
    pub(crate) fn find_possible_numbers(fields: &[Vec<u8>], row: usize, col: usize) -> Vec<u8> {
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
        for rows in fields.iter().skip(start_row).take(3) {
            for field in rows.iter().skip(start_col).take(3) {
                if *field != 0 {
                    if let Some(pos) = possible_numbers.iter().position(|&x| x == *field) {
                        possible_numbers.remove(pos);
                    }
                }
            }
        }
        
        possible_numbers
    }
}
