use crate::vector::*;
use std::fmt;
use std::ops::{Index, IndexMut};

/// A triangle comprised of three vectors with coordinates of type `T` in `N`-dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Triangle<const N: usize, T: Clone = VectorPointType> {
    /// The vectors making up the triangle.
    vecs: [Vector<N, T>; 3],
    /// The luminance of the triangle.
    lum: f32,
}

// Triangle implementation
impl<const N: usize, T: Clone> Triangle<N, T> {
    /// Create a new triangle from three vectors.
    ///
    /// `vecs`: an array of the vectors.
    /// `lum`: the luminance value.
    ///
    /// Returns the new triangle.
    pub fn new(vecs: [Vector<N, T>; 3], lum: f32) -> Self {
        Self { vecs, lum }
    }

    /// Get the vectors making up the triangle.
    ///
    /// Returns the triangle's vectors.
    pub fn vectors(&self) -> &[Vector<N, T>; 3] {
        self.vecs.as_slice().try_into().unwrap()
    }

    /// Get the luminance of the triangle.
    ///
    /// Returns the triangle's luminance value.
    pub fn get_luminance(&self) -> f32 {
        self.lum
    }

    /// Set the luminance of the triangle.
    ///
    /// `lum`: the luminance value.
    pub fn set_luminance(&mut self, lum: f32) {
        self.lum = lum;
    }

    /// Get a reference to one of the vectors making up the triangle.
    ///
    /// `index`: the index of the vector.
    ///
    /// Returns an option containing a reference to the vector, or the none variant if the index is invalid.
    pub fn get(&self, index: usize) -> Option<&Vector<N, T>> {
        self.vecs.get(index)
    }

    /// Get a mutable reference to one of the vectors making up the triangle.
    ///
    /// `index`: the index of the vector.
    ///
    /// Returns an option containing a mutable reference to the vector, or the none variant if the index is invalid.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Vector<N, T>> {
        self.vecs.get_mut(index)
    }
}

// Triangles with vectors supporting cross products and normalization
impl Triangle<3, f32> {
    /// Calculate the normal for the triangle.
    ///
    /// Returns the calculated normal.
    pub fn normal(&self) -> Vector<3, f32> {
        let a = self.vecs[1].clone() - self.vecs[0].clone();
        let b = self.vecs[2].clone() - self.vecs[0].clone();

        a.cross(&b).normalize()
    }
}

// Triangle transformation
impl<const N: usize, T: Clone + fmt::Debug> Triangle<N, T> {
    /// Transform each vector in the triangle.
    ///
    /// `f`: the closure used to map each vector.
    ///
    /// Returns the resulting triangle.
    pub fn transform<F>(&self, f: F) -> Self
    where
        F: FnMut(&Vector<N, T>) -> Vector<N, T>,
    {
        Self {
            vecs: self
                .vecs
                .iter()
                .map(f)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            lum: self.lum,
        }
    }
}

// Indexing operations for triangles
impl<const N: usize, T: Clone> Index<usize> for Triangle<N, T> {
    type Output = Vector<N, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

// Mutable indexing operations for triangles
impl<const N: usize, T: Clone> IndexMut<usize> for Triangle<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

// Convert a 3-tuple of vectors into a triangle
impl<const N: usize, T: Clone, V: Into<Vector<N, T>>> From<(V, V, V)> for Triangle<N, T> {
    fn from((x, y, z): (V, V, V)) -> Self {
        Self {
            vecs: [x.into(), y.into(), z.into()],
            lum: 1.0,
        }
    }
}

// Convert an iterator of vectors into a triangle
impl<const N: usize, T: Clone + fmt::Debug> FromIterator<Vector<N, T>> for Triangle<N, T> {
    fn from_iter<I: IntoIterator<Item = Vector<N, T>>>(iter: I) -> Self {
        let mut vecs = Vec::new();

        for i in iter {
            vecs.push(i);
        }

        Self {
            vecs: vecs.try_into().unwrap(),
            lum: 1.0,
        }
    }
}
