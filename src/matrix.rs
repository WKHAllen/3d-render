use crate::vector::*;
use std::ops::{Index, IndexMut, Mul};

/// The default type for a matrix element.
pub type MatrixElementType = f32;

/// A 2D matrix of size (`M`, `N`), with elements of type `T`.
#[derive(Clone, Copy, Debug)]
pub struct Matrix<const M: usize, const N: usize, T = MatrixElementType> {
    /// The 2D array of matrix elements.
    matrix: [[T; M]; N],
}

// Matrix implementation
impl<const M: usize, const N: usize, T> Matrix<M, N, T> {
    /// Get a reference to an element in the matrix.
    ///
    /// `x`: the x value of the element's index.
    /// `y`: the y value of the element's index.
    ///
    /// Returns an option containing a reference to the element, or the none variant if the (`x`, `y`) index is invalid.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.matrix.get(x)?.get(y)
    }

    /// Get a mutable reference to an element in the matrix.
    ///
    /// `x`: the x value of the element's index.
    /// `y`: the y value of the element's index.
    ///
    /// Returns an option containing a mutable reference to the element, or the none variant if the (`x`, `y`) index is invalid.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.matrix.get_mut(x)?.get_mut(y)
    }
}

// Matrix initialization with a fill value
impl<const M: usize, const N: usize, T: Copy> Matrix<M, N, T> {
    /// Create a matrix, filling it with a default value.
    ///
    /// `value`: the default value with which to fill the matrix.
    ///
    /// Returns the new matrix.
    pub fn fill(value: T) -> Self {
        Self {
            matrix: [[value; M]; N],
        }
    }
}

// Matrix initialization with default value
impl<const M: usize, const N: usize, T: Default + Copy> Matrix<M, N, T> {
    /// Create a matrix, filling it with `T`'s default value.
    ///
    /// Returns the new matrix.
    pub fn new() -> Self {
        Self {
            matrix: [[Default::default(); M]; N],
        }
    }
}

// Indexing operations for matrices
impl<const M: usize, const N: usize, T> Index<(usize, usize)> for Matrix<M, N, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index.0, index.1).unwrap()
    }
}

// Mutable indexing operations for matrices
impl<const M: usize, const N: usize, T> IndexMut<(usize, usize)> for Matrix<M, N, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}

// Implementation for multiplying a matrix with a vector, primarily to be used for projection matrices
impl Mul<Vector<3>> for Matrix<4, 4, f32> {
    type Output = Vector<3>;

    fn mul(self, rhs: Vector<3>) -> Self::Output {
        let x =
            rhs[0] * self[(0, 0)] + rhs[1] * self[(1, 0)] + rhs[2] * self[(2, 0)] + self[(3, 0)];
        let y =
            rhs[0] * self[(0, 1)] + rhs[1] * self[(1, 1)] + rhs[2] * self[(2, 1)] + self[(3, 1)];
        let z =
            rhs[0] * self[(0, 2)] + rhs[1] * self[(1, 2)] + rhs[2] * self[(2, 2)] + self[(3, 2)];
        let w =
            rhs[0] * self[(0, 3)] + rhs[1] * self[(1, 3)] + rhs[2] * self[(2, 3)] + self[(3, 3)];

        let mut out_vec = Vector::from([x, y, z]);

        if w != 0.0 {
            out_vec[0] /= w;
            out_vec[1] /= w;
            out_vec[2] /= w;
        }

        out_vec
    }
}

// Square matrix implementation
impl<const N: usize> Matrix<N, N, f32> {
    /// Create a new identity matrix.
    ///
    /// Returns the new identity matrix.
    pub fn identity() -> Matrix<N, N, f32> {
        let mut matrix = Self::fill(0.0);

        for i in 0..N {
            matrix[(i, i)] = 1.0;
        }

        matrix
    }
}
