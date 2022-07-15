// Import all local modules
mod color;
mod game_window;
mod matrix;
mod mesh;
mod screen;
mod transform;
mod triangle;
mod vector;

// Make all module exports visible
pub use color::*;
pub use game_window::*;
pub use matrix::*;
pub use mesh::*;
pub use screen::*;
pub use transform::*;
pub use triangle::*;
pub use vector::*;

use std::time::Instant;

/// Count frames per second.
struct FpsCounter {
    current_count: usize,
    count_since_last_update: usize,
    last_update: Instant,
}

impl FpsCounter {
    /// Create a new FPS counter.
    pub fn new() -> Self {
        Self {
            current_count: 0,
            count_since_last_update: 0,
            last_update: Instant::now(),
        }
    }

    /// Get the current FPS count.
    pub fn count(&self) -> usize {
        self.current_count
    }

    /// Update the FPS count.
    ///
    /// Returns whether the count has changed.
    pub fn update(&mut self) -> bool {
        self.count_since_last_update += 1;
        let now = Instant::now();

        if now.duration_since(self.last_update).as_micros() >= 1_000_000 {
            self.current_count = self.count_since_last_update;
            self.count_since_last_update = 0;
            self.last_update = now;

            true
        } else {
            false
        }
    }
}

fn main() {
    // Initialize the game window
    let mut window = GameWindow::new(GameWindowOptions {
        title: "Render 3D",
        width: 800,
        height: 600,
        shadow_intensity: 0.5,
        ticks_per_second: 30,
        ..Default::default()
    })
    .unwrap();

    // A mesh of a cube
    let cube: Mesh<3, f32> = Mesh::from(vec![
        // South face
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        // East face
        ([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]),
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]),
        // North face
        ([1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]),
        // West face
        ([0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]),
        ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]),
        // Top face
        ([0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]),
        // Bottom face
        ([1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]),
        ([1.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
    ]);

    // The camera
    let camera = Vector::from([0.0, 0.0, -1.0]);

    // The light
    let light = Vector::from([0.0, 0.0, -1.0]).normalize();

    // The angle of rotation for the cube
    let mut theta = 0.0;

    // Initialize the FPS counter.
    let mut fps = FpsCounter::new();

    // Game window loop
    while window.open() {
        // Fill background with dark blue
        window.fill((0, 0, 127));

        // Transform and project the cube
        let cube_projected = window
            .mesh_transformation(&cube)
            // .translate([-0.5, -0.5, -0.5])
            .rotate_z(theta)
            .rotate_x(0.5 * theta)
            .translate([0.0, 0.0, 2.0])
            .normalize_filter(&camera)
            .apply_luminance(&light)
            .project()
            .translate([1.0, 1.0])
            .scale([0.5, 0.5])
            .scale([window.get_width() as f32, window.get_height() as f32])
            .mesh();

        // Draw the transformed, projected cube
        window.fill_mesh(&cube_projected, (255, 255, 255));
        // Trigger a game window tick
        window.update().unwrap();

        // Update the cube's rotation angle
        theta = 0.5 * window.elapsed().as_secs_f32();

        // Update the FPS and show the count ever second
        if fps.update() {
            println!("{} FPS", fps.count());
        }
    }
}
