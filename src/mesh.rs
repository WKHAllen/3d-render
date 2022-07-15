use crate::triangle::*;
use crate::vector::*;

/// An object mesh comprised of triangles with type `T` `N`-dimensional vectors.
#[derive(Clone, Debug)]
pub struct Mesh<const N: usize = 3, T: Clone = VectorPointType> {
    /// The triangles making up the mesh.
    tris: Vec<Triangle<N, T>>,
}

// Mesh implementation
impl<const N: usize, T: Clone> Mesh<N, T> {
    /// Create a new mesh from a collection of triangles.
    ///
    /// `tris`: a vector of triangles.
    ///
    /// Returns the new mesh.
    pub fn new(tris: Vec<Triangle<N, T>>) -> Self {
        Self { tris }
    }

    /// Get the triangles making up the mesh.
    ///
    /// Returns the mesh's triangles.
    pub fn triangles(&self) -> &[Triangle<N, T>] {
        self.tris.as_slice()
    }

    /// Transform each triangle in the mesh.
    ///
    /// `f`: the closure used to map each triangle.
    ///
    /// Returns the resulting mesh.
    pub fn transform<F>(&self, f: F) -> Self
    where
        F: FnMut(&Triangle<N, T>) -> Triangle<N, T>,
    {
        Self {
            tris: self.tris.iter().map(f).collect(),
        }
    }
}

// Convert a vector of triangles into a mesh
impl<const N: usize, T: Clone, U: Into<Triangle<N, T>>> From<Vec<U>> for Mesh<N, T> {
    fn from(triangles: Vec<U>) -> Self {
        Self {
            tris: triangles.into_iter().map(|x| x.into()).collect(),
        }
    }
}

// Convert an iterator of triangles into a mesh
impl<const N: usize, T: Clone> FromIterator<Triangle<N, T>> for Mesh<N, T> {
    fn from_iter<I: IntoIterator<Item = Triangle<N, T>>>(iter: I) -> Self {
        let mut tris = Vec::new();

        for i in iter {
            tris.push(i);
        }

        Self { tris }
    }
}
