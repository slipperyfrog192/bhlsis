// convolves two matrices over each other and returns the result and resulting size
fn convolve(a: &Vec<f64>, (nrows_a, ncols_a): (usize, usize), b: &Vec<f64>, (nrows_b, ncols_b): (usize, usize)) -> (Vec<f64>, (usize, usize)) {
    
    // ensures that one of the matrices fits into the other
    if (nrows_a > nrows_b) & !(ncols_a >= ncols_b) {
        panic!("one matrix must be at least larger than the other in every dimension");
    }
    if (nrows_a < nrows_b) & !(ncols_a <= ncols_b) {
        panic!("one matrix must be at least larger than the other in every dimension");
    }

    // the matrix to be returned
    let mut c: Vec<f64> = Vec::new();
    let c_shape: (usize, usize) = (((nrows_a as i32) - (nrows_b as i32) + 1).abs() as usize, ((ncols_a as i32) - (ncols_b as i32) + 1).abs() as usize);

    // sets the new matrix
    for i in 0..c_shape.0 * c_shape.1 {
        let row: usize = i / c_shape.1;
        let col: usize = i % c_shape.1;
        let mut sum: f64 = 0.;
        for j in 0..b.len() {
            sum += b[j] * a[(row * ncols_a) + (col) + (j % ncols_b) + (j / ncols_b * ncols_a)];
        }
        c.push(sum);
    }
    
    // return a tuple of the matrix in vector form and the shape
    (c, c_shape)
    
}


fn main() {
    let a: Vec<f64> = vec![1., 0., 0., 1., 1., 0., 1., 0., 0., 0., 1., 1.];
    let a_shape: (usize, usize) = (4, 3);

    let b: Vec<f64> = vec![1., 0.];
    let b_shape: (usize, usize) = (1, 2);

    let (c, c_shape): (Vec<f64>, (usize, usize)) = convolve(&a, a_shape, &b, b_shape);

    println!("c = {:?}", c);
    println!("c_shape = {:?}", c_shape);
}