// convolves two matrices over each other and returns the result and resulting size
fn convolve(a: &Vec<f64>, a_shape: (usize, usize), b: &Vec<f64>, b_shape: (usize, usize), _strides: (u32, u32), padding: &str) -> (Vec<f64>, (usize, usize)) {
    // clones a and b into input and filter so that input can be mutuable if padding == "same"
    let mut input: Vec<f64> = a.clone();
    let mut input_shape: (usize, usize) = a_shape;
    let filter: Vec<f64> = b.clone();
    let filter_shape: (usize, usize) = b_shape;

    // valid does not do any padding but needs to check that the filter fits into the input in every dimension
    // same does padding such that the output dimensions (c_shape) are the same as the input dimensions (a_shape)
    if padding == "valid" {
        if input_shape.0 < filter_shape.0 || input_shape.1 < filter_shape.1 {
            panic!("filter must fit into input in everydirection");
        }
    } else if padding == "same" {
        // temp vector to set input to
        let mut temp: Vec<f64> = Vec::new();

        // padding the rows and adding the values of input
        let padding_width: u32 = (filter_shape.1 as u32) - 1;
        for i in 0..input_shape.0 * input_shape.1 {
            if i % input_shape.1 == 0 {
                for _ in 0..padding_width / 2 {
                    temp.push(0.);
                }
            }
            temp.push(input[i]);
            if (i + 1) % input_shape.1 == 0 {
                for _ in 0..padding_width / 2 + padding_width % 2 {
                    temp.push(0.);
                }
            }
        }

        // padding the columns
        let padding_height: u32 = filter_shape.0 as u32 - 1;
        for _ in 0..padding_height / 2 * (input_shape.1 as u32 + padding_width) {
            temp.insert(0, 0.);
        }
        for _ in 0..(padding_height / 2 + padding_height % 2) * (input_shape.1 as u32 + padding_width) {
            temp.push(0.);
        }

        // sets input and input shape
        input = temp;
        input_shape.0 += padding_height as usize;
        input_shape.1 += padding_width as usize;
    }
    
    // the matrix to be returned
    let mut c: Vec<f64> = Vec::new();
    let c_shape: (usize, usize) = ((input_shape.0 as i32 - filter_shape.0 as i32 + 1) as usize, (input_shape.1 as i32 - filter_shape.1 as i32 + 1) as usize);

    // sets the new matrix
    for i in 0..c_shape.0 * c_shape.1 {
        let row: usize = i / c_shape.1;
        let col: usize = i % c_shape.1;
        let mut sum: f64 = 0.;
        for j in 0..filter.len() {
            sum += filter[j] * input[(row * input_shape.1) + (col) + (j % filter_shape.1) + (j / filter_shape.1 * input_shape.1)];
        }
        c.push(sum);
    }
    
    // return input tuple of the matrix in vector form and the shape
    (c, c_shape)
}


fn main() {
    // let a: Vec<f64> = vec![1., 0., 0., 1., 1., 0., 1., 0., 0., 0., 1., 1., 0., 1., 1., 1.];
    let a: Vec<f64> = vec![1.; 16];
    let a_shape: (usize, usize) = (4, 4);

    // let b: Vec<f64> = vec![1., 0., 0., 1.];
    let b: Vec<f64> = vec![1.; 4];
    let b_shape: (usize, usize) = (2, 2);

    let (c, c_shape): (Vec<f64>, (usize, usize)) = convolve(&a, a_shape, &b, b_shape, (1, 1), "same");

    println!("c = {:?}", c);
    println!("c_shape = {:?}", c_shape);
}