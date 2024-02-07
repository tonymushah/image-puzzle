use nalgebra::{DMatrix, Scalar};
use rand::Rng;

pub fn rand_matrix<T: Scalar + Clone>(matrix: &DMatrix<T>) -> DMatrix<T> {
    let mut matrix = matrix.clone();
    let mut rng = rand::thread_rng();
    let swap_times = rng.gen_range(1, matrix.len());
    for _ in 0..swap_times {
        let x1 = rng.gen_range(0, matrix.column_iter().count() - 1);
        let y1 = rng.gen_range(0, matrix.row_iter().count() - 1);
        
        let x2 = rng.gen_range(0, matrix.column_iter().count() - 1);
        let y2 = rng.gen_range(0, matrix.row_iter().count() - 1);
        
        matrix.swap((y1, x1), (y2, x2));
    }
    matrix
}