// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
use rand::Rng;
use ark_bls12_381::Fq;
use ndarray::prelude::*;

struct Freivald {
    x: Array2<Fq>// Array/Vec of Fq,
}

impl Freivald {
    // TODO: Create constructor for object
    fn new(array_size: usize) -> Self {
        // todo!();
        // Generate random number
        // Populate vector with values r^i for i=0..matrix_size
        // Return freivald value with this vector as its x value
        let mut x = Array2::<Fq>::ones((array_size, 1));
        let mut rng = rand::thread_rng();

        for i in 0..array_size {
            let mut r: Fq = rng.gen();
            x[[i,0]] = r;
            r *= x[[i, 0]];
        }
        Freivald { x }
    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));
        // todo!();
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
        let abx = matrix_a.dot(&matrix_b.dot(&self.x));
        let cx = supposed_ab.dot(&self.x);

        // Check if the matrix multiplication on the left is same as the right
        abx == cx
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    // todo!()
    let a = Array2::<Fq>::zeros((2, 2));
    let b = Array2::<Fq>::zeros((2, 2));
    let c = Array2::<Fq>::zeros((2, 2));

    if Freivald::verify_once(&a, &b, &c) == true {
        println!("Matrix a * b is c");
    } else {
        println!("Matrix a * b isn't c");
    }
}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    // todo!()
    if  matrix_a.nrows() == matrix_b.nrows() && 
        matrix_b.nrows() == supposed_ab.nrows() &&
        matrix_a.ncols() == matrix_b.ncols() && 
        matrix_b.ncols() == supposed_ab.ncols() {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    fn new_matrix(array_size: usize) -> Array2<Fq> {
        // todo!();
        // Generate random number
        // Populate vector with values r^i for i=0..matrix_size
        // Return freivald value with this vector as its x value
        let mut x = Array2::<Fq>::ones((array_size, array_size));
        let mut rng = rand::thread_rng();

        for i in 0..array_size {
            for j in 0..array_size {
                let mut r: Fq = rng.gen();
                x[[i,j]] = r;
                r *= x[[i, j]];
            }
        }
        x     
    }

    fn pow2(m: &Array2<Fq>) -> Array2<Fq> {
        m.dot(m)
    }

    lazy_static! {
        // todo!("add matrices types and values")
        static ref MATRIX_A: Array2<Fq>/* Type of matrix. Values should be fq */ = new_matrix(200)/* arbitrary matrix */;
        static ref MATRIX_A_DOT_A: Array2<Fq>/* Type of matrix. Values should be fq */ = pow2(&MATRIX_A)/* Correct result of A * A */;
        static ref MATRIX_B: Array2<Fq>/* Type of matrix. Values should be fq */ = new_matrix(200)/* arbitrary matrix */;
        static ref MATRIX_B_DOT_B: Array2<Fq>/* Type of matrix. Values should be fq */ = pow2(&MATRIX_B)/* Correct result of B * B */;
        static ref MATRIX_C: Array2<Fq>/* Type of matrix. Values should be fq */ = new_matrix(200)/* arbitrary LARGE matrix (at least 200, 200)*/;
        static ref MATRIX_C_DOT_C: Array2<Fq>/* Type of matrix. Values should be fq */ = pow2(&MATRIX_C)/* Correct result of C * C */;
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] matrix_b: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] supposed_ab: &Array2<Fq>/* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] b: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] c: &Array2<Fq>/* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
