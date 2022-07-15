use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Game screen buffer abstraction.
#[derive(Clone, Debug)]
pub struct Screen {
    /// The screen width in pixels.
    width: usize,
    /// The screen height in pixels.
    height: usize,
    /// The pixel buffer as a collection of unsigned 32-bit integers, each representing a pixel's color.
    buffer: Vec<u32>,
}

// Screen implementation
impl Screen {
    /// Create a new screen buffer.
    ///
    /// `width`: the width of the screen in pixels.
    /// `height`: the height of the screen in pixels.
    ///
    /// Returns the new screen buffer, filled with black.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    /// Get a reference to a pixel in the screen buffer.
    ///
    /// `x`: the x coordinate of the pixel.
    /// `y`: the y coordinate of the pixel.
    ///
    /// Returns an option containing a reference to the pixel, or the none variant if the (`x`, `y`) index is invalid.
    pub fn get(&self, x: usize, y: usize) -> Option<&u32> {
        if x < self.width && y < self.height {
            self.buffer.get(y * self.width + x)
        } else {
            None
        }
    }

    /// Get a mutable reference to a pixel in the screen buffer.
    ///
    /// `x`: the x coordinate of the pixel.
    /// `y`: the y coordinate of the pixel.
    ///
    /// Returns an option containing a mutable reference to the pixel, or the none variant if the (`x`, `y`) index is invalid.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        if x < self.width && y < self.height {
            self.buffer.get_mut(y * self.width + x)
        } else {
            None
        }
    }
}

// Indexing operations for screen buffers
impl Index<(usize, usize)> for Screen {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index.0, index.1).unwrap()
    }
}

// Mutable indexing operations for screen buffers
impl IndexMut<(usize, usize)> for Screen {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}

// Dereferencing operations for screen buffers
impl Deref for Screen {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

// Mutable dereferencing operations for screen buffers
impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
