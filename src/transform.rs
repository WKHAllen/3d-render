use crate::matrix::*;
use crate::mesh::*;
use crate::triangle::*;
use crate::vector::*;
use std::fmt;
use std::ops::{Add, Mul};

/// Generate an x rotation matrix.
///
/// `angle`: the angle of rotation in radians around the x-axis.
///
/// Returns the 4x4 x rotation matrix.
fn generate_x_rotation_matrix(angle: f32) -> Matrix<4, 4, f32> {
    let mut x_rotation_matrix = Matrix::<4, 4, f32>::identity();

    x_rotation_matrix[(1, 1)] = angle.cos();
    x_rotation_matrix[(1, 2)] = angle.sin();
    x_rotation_matrix[(2, 1)] = -angle.sin();
    x_rotation_matrix[(2, 2)] = angle.cos();

    x_rotation_matrix
}

/// Generate a y rotation matrix.
///
/// `angle`: the angle of rotation in radians around the y-axis.
///
/// Returns the 4x4 y rotation matrix.
fn generate_y_rotation_matrix(angle: f32) -> Matrix<4, 4, f32> {
    let mut y_rotation_matrix = Matrix::<4, 4, f32>::identity();

    y_rotation_matrix[(0, 0)] = angle.cos();
    y_rotation_matrix[(0, 2)] = -angle.sin();
    y_rotation_matrix[(2, 0)] = angle.sin();
    y_rotation_matrix[(2, 2)] = angle.cos();

    y_rotation_matrix
}

/// Generate a z rotation matrix.
///
/// `angle`: the angle of rotation in radians around the z-axis.
///
/// Returns the 4x4 z rotation matrix.
fn generate_z_rotation_matrix(angle: f32) -> Matrix<4, 4, f32> {
    let mut z_rotation_matrix = Matrix::<4, 4, f32>::identity();

    z_rotation_matrix[(0, 0)] = angle.cos();
    z_rotation_matrix[(0, 1)] = angle.sin();
    z_rotation_matrix[(1, 0)] = -angle.sin();
    z_rotation_matrix[(1, 1)] = angle.cos();

    z_rotation_matrix
}

/// A mesh transformation comprised of triangles with type `T` `N`-dimensional vectors.
#[derive(Clone, Debug)]
pub struct Transform<const N: usize, T: Clone = VectorPointType> {
    /// The triangles making up the mesh.
    mesh: Mesh<N, T>,
    /// The screen's projection matrix.
    projection_matrix: Matrix<4, 4, f32>,
}

// Transformation implementation
impl<const N: usize, T: Clone> Transform<N, T> {
    /// Create a new mesh transformation.
    ///
    /// `mesh`: the mesh to transform.
    /// `projection_matrix`: the projection matrix for the screen.
    ///
    /// Returns the new mesh transformation.
    pub fn new(mesh: &Mesh<N, T>, projection_matrix: &Matrix<4, 4, f32>) -> Self {
        Self {
            mesh: mesh.clone(),
            projection_matrix: projection_matrix.clone(),
        }
    }

    /// Turn the transformation back into a mesh.
    ///
    /// Returns the transformed mesh.
    pub fn mesh(&self) -> Mesh<N, T> {
        Mesh::from(self.mesh.clone())
    }
}

// Translation implementation
impl<const N: usize, T: Clone + fmt::Debug + Add<Output = T>> Transform<N, T> {
    /// Translate the mesh.
    ///
    /// `values`: the amount by which to translate the mesh in each dimension.
    ///
    /// Returns the translated mesh transformation.
    pub fn translate(&mut self, values: [T; N]) -> &mut Self {
        self.mesh = self.mesh.transform(|triangle| {
            triangle.transform(|vector| {
                let mut new_vector = vector.clone();

                for i in 0..N {
                    new_vector[i] = vector[i].clone() + values[i].clone();
                }

                new_vector
            })
        });

        self
    }
}

// Scaling implementation
impl<const N: usize, T: Clone + fmt::Debug + Mul<Output = T>> Transform<N, T> {
    /// Scale the mesh.
    ///
    /// `factors`: the factors by which to scale the mesh in each dimension.
    ///
    /// Returns the scaled mesh transformation.
    pub fn scale(&mut self, factors: [T; N]) -> &mut Self {
        self.mesh = self.mesh.transform(|triangle| {
            triangle.transform(|vector| {
                let mut new_vector = vector.clone();

                for i in 0..N {
                    new_vector[i] = vector[i].clone() * factors[i].clone();
                }

                new_vector
            })
        });

        self
    }
}

// Rotation and projection implementation
impl Transform<3, f32> {
    /// Rotate the mesh about the x-axis.
    ///
    /// `angle`: the angle of rotation in radians around the x-axis.
    ///
    /// Returns the rotated mesh transformation.
    pub fn rotate_x(&mut self, angle: f32) -> &mut Self {
        let x_projection_matrix = generate_x_rotation_matrix(angle);

        self.mesh = self
            .mesh
            .transform(|triangle| triangle.transform(|&vector| x_projection_matrix * vector));

        self
    }

    /// Rotate the mesh about the y-axis.
    ///
    /// `angle`: the angle of rotation in radians around the y-axis.
    ///
    /// Returns the rotated mesh transformation.
    pub fn rotate_y(&mut self, angle: f32) -> &mut Self {
        let y_projection_matrix = generate_y_rotation_matrix(angle);

        self.mesh = self
            .mesh
            .transform(|triangle| triangle.transform(|&vector| y_projection_matrix * vector));

        self
    }

    /// Rotate the mesh about the z-axis.
    ///
    /// `angle`: the angle of rotation in radians around the z-axis.
    ///
    /// Returns the rotated mesh transformation.
    pub fn rotate_z(&mut self, angle: f32) -> &mut Self {
        let z_projection_matrix = generate_z_rotation_matrix(angle);

        self.mesh = self
            .mesh
            .transform(|triangle| triangle.transform(|&vector| z_projection_matrix * vector));

        self
    }

    /// Normalize and filter out all triangles that cannot be seen. This should be called after all other transformations and before projection.
    ///
    /// `camera`: the camera.
    ///
    /// Returns the filtered mesh transformation.
    pub fn normalize_filter(&mut self, camera: &Vector<3, f32>) -> &mut Self {
        self.mesh = self
            .mesh
            .triangles()
            .to_owned()
            .into_iter()
            .filter(|triangle| triangle.normal().dot(&(triangle[0] - *camera)) < 0.0)
            .collect();

        self
    }

    /// Apply luminance to the triangles within the mesh.
    ///
    /// `light`: the position of the light source.
    ///
    /// Returns the resulting mesh transformation.
    pub fn apply_luminance(&mut self, light: &Vector<3, f32>) -> &mut Self {
        self.mesh = self.mesh.transform(|triangle| {
            let mut new_triangle = triangle.to_owned();
            new_triangle.set_luminance(triangle.normal().dot(light));
            new_triangle
        });

        self
    }

    /// Project the mesh transformation from 3D space into 2D space.
    ///
    /// Returns the projected mesh transformation.
    pub fn project(&self) -> Transform<2, f32> {
        let projected_tris = self
            .mesh
            .triangles()
            .iter()
            .map(|triangle| {
                Triangle::new(
                    triangle
                        .vectors()
                        .iter()
                        .map(|&vec| self.projection_matrix * vec)
                        .map(|vec| Vector::from([vec[0], vec[1]]))
                        .collect::<Vec<Vector<2>>>()
                        .try_into()
                        .unwrap(),
                    triangle.get_luminance(),
                )
            })
            .collect::<Vec<_>>();

        Transform {
            mesh: Mesh::from(projected_tris),
            projection_matrix: self.projection_matrix.clone(),
        }
    }
}

// Convert the mesh transformation back into a mesh.
impl<const N: usize, T: Clone + fmt::Debug + Add<Output = T>> Into<Mesh<N, T>> for Transform<N, T> {
    fn into(self) -> Mesh<N, T> {
        self.mesh()
    }
}
