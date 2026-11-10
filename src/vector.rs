#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub data: Vec<f32>,
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    pub fn add(&mut self, other: &Vector) {
        assert_eq!(self.data.len(), other.data.len());

        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }    
    }
}