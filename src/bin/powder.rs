//! Powder game binary.

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

fn main() {
    println!("Hello Powder!");

    // Initialisation.
    let w = 128;
    let h = 128;

    // Resources.
    let buffer: Vec<u32> = vec![0; w * h];
    let mut win = make_window(w, h);

    // Limit to max ~60 fps update rate
    // win.limit_update_rate(Some(std::time::Duration::from_micros(10000)));

    // Main loop.
    while win.is_open() && !win.is_key_down(Key::Escape) {
        win.update_with_buffer(&buffer, w, h)
            .expect("Failed to render.")
    }
}

// Create the main window.
fn make_window(width: usize, height: usize) -> Window {
    debug_assert!(width > 0);
    debug_assert!(height > 0);

    let window_options = WindowOptions {
        borderless: true,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Center,
        topmost: true,
        transparency: true,
    };

    Window::new("Powder Game III", width, height, window_options).unwrap_or_else(|e| {
        panic!("Could not create gui window: {}", e);
    })
}
