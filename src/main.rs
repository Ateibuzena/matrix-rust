use the_matrix_rust::matrix::Matrix;
use the_matrix_rust::vector::Vector;

fn main() {
    println!("Vectors:");

    let v = Vector::new(vec![4.0, 5.0, 6.0]);
    println!("Vector v: {:?}", v.data);
    println!("Vector v: {:?}", v);

    let u = Vector::new(vec![1.0, 2.0, 3.0]);
    println!("Vector u: {:?}", u.data);
    println!("Vector u: {:?}", u);

    let mut result = u.clone();

    result.add(&v);
    println!("Result of u + v: {:?}", result.data);
    println!("Result of u + v: {:?}", result);

    result = u.clone();
    result.sub(&v);
    println!("Result of u - v: {:?}", result.data);
    println!("Result of u - v: {:?}", result);

    result = u.clone();
    result.scl(2.0);
    println!("Result of u * 2: {:?}", result.data);
    println!("Result of u * 2: {:?}", result);

    println!("\nMatrix:");

    let mut m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    println!("Matrix m: {:?}", m.data);
    println!("Matrix m: {:?}", m);
    println!("Matrix m :");
    m.print();

    m.scl(3.0);
    println!("Result of m * 3: {:?}", m.data);
    println!("Result of m * 3: {:?}", m);
    println!("Matrix m :");
    m.print();

    // set and get
    m.set(0, 1, 10.0);
    let value = m.get(0, 1);
    println!("After setting m[0][1] to 10.0: {:?}", value);
    println!("After setting m[0][1] to 10.0: {:?}", m);
    println!("Matrix m :");
    m.print();

    let n = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    println!("Matrix n: {:?}", n.data);
    println!("Matrix n: {:?}", n);
    println!("Matrix n :");
    n.print();

    let mut result = n.clone();

    result.add(&m);
    println!("Result of n + m: {:?}", result.data);
    println!("Result of n + m: {:?}", result);
    println!("Matrix result :");
    result.print();

    result = n.clone();
    result.sub(&m);
    println!("Result of n - m: {:?}", result.data);
    println!("Result of n - m: {:?}", result);
    println!("Matrix result :");
    result.print();
}
