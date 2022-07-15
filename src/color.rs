use std::ops::{Index, IndexMut};

/// A representation of a color.
#[derive(Clone, Copy, Debug)]
pub struct Color(u8, u8, u8);

// Convert a 3-tuple into a color
impl From<(u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8)) -> Self {
        Self(color.0, color.1, color.2)
    }
}

// Convert a color into a 3-tuple
impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}

// Convert an unsigned 32-bit integer into a color
impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self::from_u8_rgb(color)
    }
}

// Convert a color into an unsigned 32-bit integer
impl Into<u32> for Color {
    fn into(self) -> u32 {
        self.to_u8_rgb()
    }
}

// Color implementation
impl Color {
    /// Instantiate a new color.
    ///
    /// `r`: the red value.
    /// `g`: the green value.
    /// `b`: the blue value.
    ///
    /// Returns the new color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Instantiate a color from an unsigned 32-bit integer.
    ///
    /// `u8_rgb`: the unsigned 32-bit integer representation of the color.
    ///
    /// Returns the new color.
    pub fn from_u8_rgb(u8_rgb: u32) -> Self {
        let (r, g, b) = ((u8_rgb >> 16) & 255, (u8_rgb >> 8) & 255, u8_rgb & 255);
        Self(r as u8, g as u8, b as u8)
    }

    /// Convert the color into an unsigned 32-bit integer.
    ///
    /// Returns the color represented as an unsigned 32-bit integer.
    pub fn to_u8_rgb(&self) -> u32 {
        let (r, g, b) = (self.0 as u32, self.1 as u32, self.2 as u32);
        (r << 16) | (g << 8) | b
    }

    /// Get a reference to a color value.
    ///
    /// `index`: the index of the color value.
    ///
    /// Returns an option containing a reference to the color value, or the none variant if the index is invalid.
    pub fn get(&self, index: usize) -> Option<&u8> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            _ => None,
        }
    }

    /// Get a mutable reference to a color value.
    ///
    /// `index`: the index of the color value.
    ///
    /// Returns an option containing a mutable reference to the color value, or the none variant if the index is invalid.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            _ => None,
        }
    }
}

// Indexing operations for colors
impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

// Mutable index operations for colors
impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
