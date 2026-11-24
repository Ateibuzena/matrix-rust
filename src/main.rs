use the_matrix_rust::vector::Vector;
use the_matrix_rust::matrix::Matrix;

fn main() {

    println!("Vectors:");

    let v = Vector::new(vec![4.0, 5.0, 6.0]);

    let mut u = Vector::new(vec![1.0, 2.0, 3.0]);
    u.add(&v);
    println!("Result of u + v: {:?}", u.data);
    println!("Result of u + v: {:?}", u);

    let mut w = Vector::new(vec![7.0, 8.0, 9.0]);
    w.sub(&v);
    println!("Result of w - v: {:?}", w.data);
    println!("Result of w - v: {:?}", w);

    let mut z = Vector::new(vec![1.0, 2.0, 3.0]);
    z.scl(2.0);
    println!("Result of z * 2: {:?}", z.data);
    println!("Result of z * 2: {:?}", z);

    println!("\nMatrices:");

    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    println!("Matrix m: {:?}", m.data);
    println!("Matrix m: {:?}", m);

    let mut n = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
    n.add(&m);
    println!("Result of n + m: {:?}", n.data);
    println!("Result of n + m: {:?}", n);

    let mut p = Matrix::new(2, 2, vec![9.0, 10.0, 11.0, 12.0]);
    p.sub(&m);
    println!("Result of p - m: {:?}", p.data);
    println!("Result of p - m: {:?}", p);

    let mut q = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    q.scl(3.0);
    println!("Result of q * 3: {:?}", q.data);
    println!("Result of q * 3: {:?}", q);

    // set and get
    let mut r = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    r.set(0, 1, 10.0);
    let value = r.get(0, 1);
    println!("After setting r[0][1] to 10.0: {:?}", value);
    println!("After setting r[0][1] to 10.0: {:?}", r);

    println!("Matrix r :");
    r.print();

}
