pub struct Location {
    pub column: u32,
    pub row: u32
}

impl Location {
    pub fn new(column: u32, row: u32) -> Self {
        Self {
            column,
            row
        }
    }

    pub fn get_column_string(&self) -> String {
        const CHARACTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        const BASE: usize = CHARACTERS.len();
        const WIDTH: usize = 7;
        
        let mut column = self.column.clone();
        let mut buffer = ['\0'; WIDTH];

        for i in 0..WIDTH {
            let remainder = (column as usize) % BASE;
            
            buffer[i] = CHARACTERS[remainder];
            column /= BASE as u32;
        }

        buffer.iter().collect::<String>()
    }

    pub fn get_row_string(&self) -> String {
        self.row.to_string()
    }

    pub fn as_string(&self) -> String {
        let mut result = self.get_column_string();
        result.push_str(&self.get_row_string());

        result
    }
}