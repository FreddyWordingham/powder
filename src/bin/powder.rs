//! Powder game binary.

use arctk::{
    args,
    file::Load,
    ord::{X, Y},
    util::{
        banner::{section, title},
        dir,
    },
};
use arctk_attr::input;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use powder::parts::World;
use std::{env::current_dir, path::PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Simulation resolution.
    res: [usize; 2],
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Powder");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, _out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");

    // Initialisation.
    let w = params.res[X];
    let h = params.res[Y];

    // Resources.
    let buffer: Vec<u32> = vec![0; w * h];
    let mut win = make_window(w, h);

    let mut world = World::new(params.res);

    // Limit to max ~60 fps update rate
    // win.limit_update_rate(Some(std::time::Duration::from_micros(10000)));

    // Main loop.
    while win.is_open() && !win.is_key_down(Key::Escape) {
        world.tick();
        world.draw(&mut buffer);

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
