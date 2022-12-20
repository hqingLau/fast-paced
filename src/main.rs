mod ball;
pub mod app;
mod render;
use piston_window::{PistonWindow, WindowSettings};



fn main() {
    let window: PistonWindow = WindowSettings::new("Hello", [app::WIDTH,app::HEIGHT])
        .exit_on_esc(true).build().unwrap();

    render::render_window(window);
}
