use the_matrix_rust::matrix::Matrix;
use the_matrix_rust::vector::Vector;

#[test]
fn test_vector_add() {
    let mut u = Vector::new(vec![2.0, 3.0]);
    let v = Vector::new(vec![5.0, 7.0]);

    u.add(&v);

    assert_eq!(u.data, vec![7.0, 10.0]);
}

#[test]
fn test_vector_sub() {
    let mut u = Vector::new(vec![2.0, 3.0]);
    let v = Vector::new(vec![5.0, 7.0]);

    u.sub(&v);

    assert_eq!(u.data, vec![-3.0, -4.0]);
}

#[test]
fn test_vector_scale() {
    let mut u = Vector::new(vec![2.0, 3.0]);

    u.scl(2.0);

    assert_eq!(u.data, vec![4.0, 6.0]);
}

#[test]
fn test_matrix_add() {
    let mut a = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]);
    let b = Matrix::new(2, 2, vec![7.0, -2.0, 4.0, 2.0]);

    a.add(&b);

    assert_eq!(a.get(0, 0), 8.0);
    assert_eq!(a.get(0, 1), 6.0);
    assert_eq!(a.get(1, 0), 1.0);
    assert_eq!(a.get(1, 1), 6.0);
}

#[test]
fn test_matrix_sub() {
    let mut a = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]);
    let b = Matrix::new(2, 2, vec![7.0, -2.0, 4.0, 2.0]);

    a.sub(&b);

    assert_eq!(a.get(0, 0), -6.0);
    assert_eq!(a.get(0, 1), -2.0);
    assert_eq!(a.get(1, 0), 5.0);
    assert_eq!(a.get(1, 1), 2.0);
}

#[test]
fn test_matrix_scale() {
    let mut a = Matrix::new(2, 2, vec![1.0, 3.0, 2.0, 4.0]);

    a.scl(2.0);

    assert_eq!(a.get(0, 0), 2.0);
    assert_eq!(a.get(0, 1), 4.0);
    assert_eq!(a.get(1, 0), 6.0);
    assert_eq!(a.get(1, 1), 8.0);
}
