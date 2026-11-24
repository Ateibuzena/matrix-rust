#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub data: Vec<f32>,
}

impl Vector {

    // Creates a new Vector with the given data.
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    // Adds another Vector to this Vector in place.
    pub fn add(&mut self, other: &Vector) {

        // Ensure both vectors have the same length before performing addition.
        assert_eq!(self.data.len(), other.data.len());

        for i in 0..self.data.len() {
            let a = &mut self.data[i];
            let b = &other.data[i];
            *a = *a + *b;
        }
    }

    // Subtracts another Vector from this Vector in place.
    pub fn sub(&mut self, other: &Vector) {

        // Ensure both vectors have the same length before performing addition.
        assert_eq!(self.data.len(), other.data.len());

        for i in 0..self.data.len() {
            let a = &mut self.data[i];
            let b = &other.data[i];
            *a = *a - *b;
        }
    }
    
    // Scales the Vector by a scalar value in place.
    pub fn scl(&mut self, scalar: f32) {
       // Ensure the scalar is not zero.
        assert!(scalar != 0.0);

        for value in &mut self.data {
            *value *= scalar;
        }
    }
}