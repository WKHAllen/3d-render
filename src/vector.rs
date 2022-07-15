use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

/// The default type for a vector point.
pub type VectorPointType = f32;

/// A vector with coordinates of type `T` in `N`-dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Vector<const N: usize, T: Clone = VectorPointType> {
    /// The vector's coordinates.
    coordinates: [T; N],
}

// Vector implementation
impl<const N: usize, T: Clone> Vector<N, T> {
    /// Get the vector's coordinates.
    ///
    /// Returns the coordinates of the vector.
    pub fn coords(&self) -> &[T; N] {
        self.coordinates.as_slice().try_into().unwrap()
    }

    /// Get a reference to the value of one of the coordinates of the vector.
    ///
    /// `index`: the index of the value in the coordinates.
    ///
    /// Returns an option containing a reference to the value, or the none variant if the index is invalid.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.coordinates.get(index)
    }

    /// Get a mutable reference to the value of one of the coordinates of the vector.
    ///
    /// `index`: the index of the value in the coordinates.
    ///
    /// Returns an option containing a mutable reference to the value, or the none variant if the index is invalid.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.coordinates.get_mut(index)
    }
}

// Vector transformation
impl<const N: usize, T: Clone + fmt::Debug> Vector<N, T> {
    /// Transform each value in the vector.
    ///
    /// `f`: the closure used to map each value.
    ///
    /// Returns the resulting vector.
    pub fn transform<F>(&self, f: F) -> Self
    where
        F: FnMut(&T) -> T,
    {
        Self {
            coordinates: self
                .coordinates
                .iter()
                .map(f)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

// 3D vector computations
impl<T: Clone + Sub<Output = T> + Mul<Output = T>> Vector<3, T> {
    /// Perform a cross product.
    ///
    /// `other`: the other vector.
    ///
    /// Returns the resulting vector after the cross product.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            coordinates: [
                self.coordinates[1].clone() * other.coordinates[2].clone()
                    - self.coordinates[2].clone() * other.coordinates[1].clone(),
                self.coordinates[2].clone() * other.coordinates[0].clone()
                    - self.coordinates[0].clone() * other.coordinates[2].clone(),
                self.coordinates[0].clone() * other.coordinates[1].clone()
                    - self.coordinates[1].clone() * other.coordinates[0].clone(),
            ],
        }
    }
}

// Vector normalization
impl<const N: usize> Vector<N, f32> {
    /// Normalize the vector.
    ///
    /// Returns the normalized vector.
    pub fn normalize(&self) -> Self {
        let length = (self.coordinates.iter().map(|x| x * x).sum::<f32>()).sqrt();

        Self {
            coordinates: self
                .coordinates
                .iter()
                .map(|value| value / length)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<const N: usize, T: Clone + Mul<Output = T> + Sum> Vector<N, T> {
    /// Compute the dot product of two vectors.
    ///
    /// `other`: the other vector.
    ///
    /// Returns the resulting dot product.
    pub fn dot(&self, other: &Self) -> T {
        (0..N)
            .into_iter()
            .map(|i| self.coordinates[i].clone() * other.coordinates[i].clone())
            .sum()
    }
}

// Indexing operations for vectors
impl<const N: usize, T: Clone> Index<usize> for Vector<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

// Mutable indexing operations for vectors
impl<const N: usize, T: Clone> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

// Convert an array into a vector
impl<const N: usize, T: Clone, U: Into<T>> From<[U; N]> for Vector<N, T>
where
    [T; N]: From<[U; N]>,
{
    fn from(coords: [U; N]) -> Self {
        Self {
            coordinates: coords.into(),
        }
    }
}

// Add vectors
impl<const N: usize, T: Clone + fmt::Debug + Add<Output = T>> Add for Vector<N, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            coordinates: (0..self.coordinates.len())
                .into_iter()
                .map(|i| self.coordinates[i].clone() + rhs.coordinates[i].clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

// Subtract vectors
impl<const N: usize, T: Clone + fmt::Debug + Sub<Output = T>> Sub for Vector<N, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            coordinates: (0..self.coordinates.len())
                .into_iter()
                .map(|i| self.coordinates[i].clone() - rhs.coordinates[i].clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
