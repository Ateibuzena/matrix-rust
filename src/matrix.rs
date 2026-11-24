#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl Matrix {

    // Creates a new Matrix with the given dimensions and data.
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        // Ensure the data length matches the specified dimensions.
        assert_eq!(data.len(), rows * cols); 

        // Initialize the Matrix with the provided rows, columns, and data.
        Self { rows, cols, data }
    }

    // Returns the index in the data vector for the given row and column.
    fn index(&self, row: usize, col: usize) -> usize {
        // Ensure the row and column indices are within bounds.
        assert!(row < self.rows && col < self.cols);

        // Calculate the index based on row-major order.
        let colum_offset = col * self.rows;

        // Calculate the row offset.
        let row_offset = row;

        // Calculate the final index in the data vector.
        let index = colum_offset + row_offset;

        // Return the calculated index.
        index
    }

    // Gets the value at the specified row and column.
    pub fn get(&self, row: usize, col: usize) -> f32 {
        // Ensure the row and column indices are within bounds.
        assert!(row < self.rows && col < self.cols);

        // Get the index for the specified row and column.
        let index = self.index(row, col);  

        // Return the value at the calculated index.
        self.data[index]
    }

    // Sets the value at the specified row and column.
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        // Ensure the row and column indices are within bounds.
        assert!(row < self.rows && col < self.cols);

        // Get the index for the specified row and column.
        let index = self.index(row, col);

        // Set the value at the calculated index.
        self.data[index] = value;
    }
    
    // Adds another Matrix to this Matrix in place.
    pub fn add(&mut self, other: &Matrix) {

        // Ensure both matrices have the same dimensions before performing addition.
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        // Iterate over each element in the matrices and perform addition.
        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }
    }

    // Subtracts another Matrix from this Matrix in place.
    pub fn sub(&mut self, other: &Matrix) { 

        // Ensure both matrices have the same dimensions before performing subtraction.
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        // Iterate over each element in the matrices and perform subtraction.
        for i in 0..self.data.len() {
            self.data[i] -= other.data[i];
        }
    }

    // Scales the Matrix by a scalar value in place.
    pub fn scl(&mut self, scalar: f32) {
        // Ensure the scalar is not zero.
        assert!(scalar != 0.0);

        // Iterate over each element in the matrix and scale it by the given scalar.
        for value in &mut self.data {
            *value *= scalar;
        }
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.get(row, col));
            }
            println!();
        }
    }
}