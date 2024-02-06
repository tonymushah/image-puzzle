use nalgebra::{base::Matrix, DMatrix, Scalar};

pub fn rand_matrix<T: Scalar>(matrix: DMatrix<T>) -> DMatrix<T> {
    let matrix = matrix.clone();
    
    //matrix.swap((), ())
    matrix
}