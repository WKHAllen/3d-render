use crate::color::*;
use crate::matrix::*;
use crate::mesh::*;
use crate::screen::*;
use crate::transform::*;
use crate::triangle::*;
use crate::vector::*;
use minifb::{Window, WindowOptions};
use std::error;
use std::fmt;
use std::time::{Duration, Instant};

/// Generate a projection matrix for the screen.
///
/// `aspect_ratio`: the screen aspect ratio.
/// `fov_rad`: the field of view in radians.
/// `near`: the view near factor.
/// `far`: the view far factor.
///
/// Returns the generated 4x4 projection matrix.
fn generate_projection_matrix(
    aspect_ratio: f32,
    fov_rad: f32,
    near: f32,
    far: f32,
) -> Matrix<4, 4, f32> {
    let mut projection_matrix = Matrix::<4, 4, f32>::fill(0.0);

    projection_matrix[(0, 0)] = aspect_ratio * fov_rad;
    projection_matrix[(1, 1)] = fov_rad;
    projection_matrix[(2, 2)] = far / (far - near);
    projection_matrix[(3, 2)] = (-far * near) / (far - near);
    projection_matrix[(2, 3)] = 1.0;
    projection_matrix[(3, 3)] = 1.0;

    projection_matrix
}

/// An error type for a game window.
#[derive(Debug)]
pub enum GameWindowError {
    /// An error from the underlying minifb implementation.
    MiniFBError(minifb::Error),
    /// A different type of error, represented as a string.
    Other(String),
}

// Convert minifb errors to game window errors
impl From<minifb::Error> for GameWindowError {
    fn from(e: minifb::Error) -> Self {
        Self::MiniFBError(e)
    }
}

// Convert strings to game window errors
impl From<&str> for GameWindowError {
    fn from(s: &str) -> Self {
        Self::Other(s.to_owned())
    }
}

// Display game window errors
impl fmt::Display for GameWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MiniFBError(e) => e.to_string(),
                Self::Other(s) => s.clone(),
            }
        )
    }
}

// Mark game window error as an implementation of the standard error trait
impl error::Error for GameWindowError {}

/// A result type for a game window.
pub type GameWindowResult<T> = Result<T, GameWindowError>;

/// Game window initialization options.
pub struct GameWindowOptions<'a> {
    /// The window title.
    pub title: &'a str,
    /// The width of the window in pixels.
    pub width: usize,
    /// The height of the window in pixels.
    pub height: usize,
    /// The view near factor.
    pub near: f32,
    /// The view far factor.
    pub far: f32,
    /// The field of view in degrees.
    pub fov: f32,
    // The intensity of low luminance values.
    pub shadow_intensity: f32,
    /// The number of ticks the game will perform per second.
    pub ticks_per_second: usize,
}

// Support default values for game window options
impl<'a> Default for GameWindowOptions<'a> {
    fn default() -> Self {
        Self {
            title: "",
            width: 800,
            height: 600,
            near: 0.1,
            far: 1000.0,
            fov: 90.0,
            shadow_intensity: 1.0,
            ticks_per_second: 60,
        }
    }
}

/// Game window abstraction.
pub struct GameWindow {
    /// A handle to the window itself.
    window: Window,
    /// The window title.
    title: String,
    /// The width of the window in pixels.
    width: usize,
    /// The height of the window in pixels.
    height: usize,
    /// The view near factor.
    near: f32,
    /// The view far factor.
    far: f32,
    /// The field of view in degrees.
    fov: f32,
    /// The intensity of low luminance values.
    shadow_intensity: f32,
    /// The number of ticks the game will perform per second.
    ticks_per_second: usize,
    /// The number of total ticks the game has performed.
    tick_count: u64,
    /// The time at which the window was opened.
    start_time: Instant,
    /// The screen buffer.
    buffer: Screen,
    /// The projection matrix for the screen.
    projection_matrix: Matrix<4, 4, f32>,
}

// Game window implementation
impl GameWindow {
    /// Create a new game window.
    ///
    /// `options`: the game window options.
    ///
    /// Returns a result containing the new game window instance, or the error variant if the window could not be created.
    pub fn new(options: GameWindowOptions) -> GameWindowResult<Self> {
        // Attempt to create the underlying window itself
        let window = Window::new(
            options.title,
            options.width,
            options.height,
            WindowOptions {
                resize: true,
                ..Default::default()
            },
        )?;

        let mut game_window = Self {
            window,
            title: options.title.to_owned(),
            width: options.width,
            height: options.height,
            near: options.near,
            far: options.far,
            fov: options.fov,
            shadow_intensity: options.shadow_intensity,
            ticks_per_second: options.ticks_per_second,
            tick_count: 0,
            start_time: Instant::now(),
            buffer: Screen::new(options.width, options.height),
            projection_matrix: Matrix::new(),
        };

        // Update the view parameters
        game_window.update_view();

        Ok(game_window)
    }

    /// Check if the window is open.
    ///
    /// Returns whether the window is open.
    pub fn open(&self) -> bool {
        self.window.is_open()
    }

    /// Get the window title.
    ///
    /// Returns the window title.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Set the window title.
    ///
    /// `title`: the new window title.
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
        self.title = title.to_owned();
    }

    /// Get the width of the window.
    ///
    /// Returns the width of the window in pixels.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Set the width of the window.
    ///
    /// `width`: the new window width in pixels.
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.update_view();
    }

    /// Get the height of the window.
    ///
    /// Returns the height of the window in pixels.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Set the height of the window.
    ///
    /// `height`: the new window height in pixels.
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.update_view();
    }

    /// Get the width and height of the window.
    ///
    /// Returns a tuple of the width and height of the window in pixels.
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Set the width and height of the window.
    ///
    /// `width`: the new window width in pixels.
    /// `height`: the new window height in pixels.
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.update_view();
    }

    /// Get the view near factor.
    ///
    /// Returns the near factor.
    pub fn get_near(&self) -> f32 {
        self.near
    }

    /// Set the view near factor.
    ///
    /// `near`: the new near factor.
    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.update_view();
    }

    /// Get the view far factor.
    ///
    /// Returns the far factor.
    pub fn get_far(&self) -> f32 {
        self.far
    }

    /// Set the view far factor.
    ///
    /// `far`: the new far factor.
    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.update_view();
    }

    /// Get the field of view.
    ///
    /// Returns the field of view in degrees.
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    /// Set the field of view.
    ///
    /// `fov`: the new field of view in degrees.
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update_view();
    }

    /// Get the number of ticks per second.
    ///
    /// Returns the current number of ticks per second.
    pub fn get_ticks_per_second(&self) -> usize {
        self.ticks_per_second
    }

    /// Set the number of ticks per second.
    ///
    /// `ticks_per_second`: the new number of ticks per second.
    pub fn set_ticks_per_second(&mut self, ticks_per_second: usize) {
        self.ticks_per_second = ticks_per_second;
    }

    /// Update the screen view buffer and projection matrix. This should be called after any update to screen size, field of view, near or far factors, etc.
    fn update_view(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);
        let fov_rad = 1.0 / ((self.fov / 2.0).to_radians()).tan();

        self.projection_matrix =
            generate_projection_matrix(aspect_ratio, fov_rad, self.near, self.far);

        self.buffer = Screen::new(self.width, self.height);
    }

    /// Get the number of ticks that have occurred so far.
    ///
    /// Returns the total number of ticks.
    pub fn ticks(&self) -> u64 {
        self.tick_count
    }

    /// Get the amount of time that has passed since the window was opened.
    ///
    /// Returns the duration since the window was created.
    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }

    /// Asychronously await the next game tick.
    pub async fn await_next_tick(&self) {
        // TODO: find an async way to wait a certain amount of time
        todo!()
    }

    /// Update the window. This should be called once per game tick to ensure the screen is drawn and window events are captured.
    pub fn update(&mut self) -> GameWindowResult<()> {
        self.tick_count += 1;

        let (window_width, window_height) = self.window.get_size();

        if window_width != self.width || window_height != self.height {
            self.set_size(window_width, window_height);
        }

        self.window
            .update_with_buffer(&*self.buffer, self.width, self.height)?;

        Ok(())
    }

    /// Apply a luminance to a color.
    ///
    /// `color`: the color.
    /// `luminance`: the luminance value.
    ///
    /// Returns the resulting color with luminance applied.
    fn apply_color_luminance(&self, color: Color, luminance: f32) -> Color {
        let lum = 1.0 - ((1.0 - luminance) * self.shadow_intensity);

        Color::new(
            ((color[0] as f32) * lum).round() as u8,
            ((color[1] as f32) * lum).round() as u8,
            ((color[2] as f32) * lum).round() as u8,
        )
    }

    /// Fill the screen with a single color.
    ///
    /// `color`: the color with which to fill the screen.
    pub fn fill<C: Into<Color>>(&mut self, color: C) {
        *self.buffer = vec![color.into().into(); self.width * self.height];
    }

    /// Draw a pixel to the screen.
    ///
    /// `p`: the point on the screen to draw.
    /// `color`: the color of the point.
    pub fn draw<V: Into<Vector<2>>, C: Into<Color>>(&mut self, p: V, color: C) {
        let p: Vector<2> = p.into();
        let color: Color = color.into();

        let px = p[0].round() as usize;
        let py = p[1].round() as usize;

        if let Some(point) = self.buffer.get_mut(px, py) {
            *point = color.into();
        }
    }

    /// Draw a line to the screen.
    ///
    /// `p1`: one endpoint of the line.
    /// `p2`: the other endpoint of the line.
    /// `color`: the color of the line.
    pub fn draw_line<V: Into<Vector<2>>, C: Into<Color> + Copy>(&mut self, p1: V, p2: V, color: C) {
        let p1: Vector<2> = p1.into();
        let p2: Vector<2> = p2.into();

        let p1x = p1[0];
        let p1y = p1[1];
        let p2x = p2[0];
        let p2y = p2[1];
        let dx = p1x - p2x;
        let dy = p1y - p2y;
        let steps = dx.abs().max(dy.abs()).round() as usize;

        for i in 0..=steps {
            let steps_progress = (i as f32) / (steps as f32);
            let x = p1x + ((p2x - p1x) * steps_progress);
            let y = p1y + ((p2y - p1y) * steps_progress);

            self.draw([x, y], color);
        }
    }

    /// Draw a shape to the screen. This will panic if called with less than two points.
    ///
    /// `points`: a list of points to draw.
    /// `color`: the color of the shape.
    pub fn draw_shape<V: Into<Vector<2>> + Clone, C: Into<Color> + Copy>(
        &mut self,
        points: Vec<V>,
        color: C,
    ) {
        if points.len() < 2 {
            panic!("expected at least two points");
        }

        for i in 0..points.len() - 1 {
            self.draw_line(points[i].clone(), points[i + 1].clone(), color);
        }

        self.draw_line(
            points.first().unwrap().clone(),
            points.last().unwrap().clone(),
            color,
        );
    }

    /// Draw a mesh to the screen.
    ///
    /// `mesh`: the mesh to draw.
    /// `color`: the color of the mesh.
    pub fn draw_mesh<C: Into<Color> + Copy>(&mut self, mesh: &Mesh<2>, color: C) {
        for triangle in mesh.triangles() {
            self.draw_shape(triangle.vectors().to_vec(), color);
        }
    }

    /// Fill in a triangle on the screen.
    ///
    /// `triangle`: the triangle to fill.
    /// `color`: the color of the triangle.
    pub fn fill_triangle<C: Into<Color> + Copy>(&mut self, triangle: &Triangle<2>, color: C) {
        let color_lum = self.apply_color_luminance(color.into(), triangle.get_luminance());

        let mut vecs = triangle.vectors().to_vec();

        let min_x_index = (0..3)
            .into_iter()
            .reduce(|min, current| {
                if vecs[min][0] < vecs[current][0] {
                    min
                } else {
                    current
                }
            })
            .unwrap();
        let max_x_index = (0..3)
            .into_iter()
            .reduce(|min, current| {
                if vecs[min][0] > vecs[current][0] {
                    min
                } else {
                    current
                }
            })
            .unwrap();

        let vec_left = vecs.remove(min_x_index);
        let vec_right = vecs.remove(if max_x_index < min_x_index {
            max_x_index
        } else {
            max_x_index - 1
        });
        let vec_mid = vecs.remove(0);

        let x_left = vec_left[0].round() as usize;
        let x_right = vec_right[0].round() as usize;
        let x_mid = vec_mid[0].round() as usize;

        let steps_full = x_right - x_left;
        let steps_left = x_mid - x_left;
        let steps_right = x_right - x_mid;

        for i in 0..=steps_left {
            let steps_progress_left = (i as f32) / (steps_left as f32);
            let steps_progress_full = (i as f32) / (steps_full as f32);
            let x = vec_left[0] + ((vec_mid[0] - vec_left[0]) * steps_progress_left);
            let y1 = vec_left[1] + ((vec_mid[1] - vec_left[1]) * steps_progress_left);
            let y2 = vec_left[1] + ((vec_right[1] - vec_left[1]) * steps_progress_full);

            self.draw_line([x, y1], [x, y2], color_lum);
        }

        for i in 0..=steps_right {
            let steps_progress_right = (i as f32) / (steps_right as f32);
            let steps_progress_full = ((steps_left as f32) + (i as f32)) / (steps_full as f32);
            let x = vec_mid[0] + ((vec_right[0] - vec_mid[0]) * steps_progress_right);
            let y1 = vec_mid[1] + ((vec_right[1] - vec_mid[1]) * steps_progress_right);
            let y2 = vec_left[1] + ((vec_right[1] - vec_left[1]) * steps_progress_full);

            self.draw_line([x, y1], [x, y2], color_lum);
        }
    }

    /// Fill in a mesh on the screen.
    ///
    /// `mesh`: the mesh to fill.
    /// `color`: the color of the mesh.
    pub fn fill_mesh<C: Into<Color> + Copy>(&mut self, mesh: &Mesh<2>, color: C) {
        for triangle in mesh.triangles() {
            self.fill_triangle(triangle, color);
        }
    }

    /// Generate a transformation for a mesh based on the window's projection matrix.
    ///
    /// `mesh`: the mesh to transform.
    ///
    /// Returns a transformation for the mesh.
    pub fn mesh_transformation<T: Clone>(&self, mesh: &Mesh<3, T>) -> Transform<3, T> {
        Transform::new(mesh, &self.projection_matrix)
    }
}
