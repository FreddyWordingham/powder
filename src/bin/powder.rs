//! Powder game binary.

use arctk::{
    args,
    fs::File,
    ord::{X, Y},
    report,
    util::{
        banner::{section, sub_section, title},
        dir, term,
    },
};
use arctk_attr::input;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use powder::parts::{CellSize, World};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Backup print width if the terminal width can not be determined.
const BACKUP_TERM_WIDTH: usize = 80;

/// Input parameter structure.
#[input]
struct Parameters {
    /// Ticks per update frame.
    tpf: usize,
    /// Frames per second.
    fps: u64,
    /// Simulation resolution.
    res: [usize; 2],
    /// Cell size.
    cell_size: CellSize,
}

fn main() {
    let term_width = term::width(BACKUP_TERM_WIDTH);
    title(term_width, "Powder");

    let (in_dir, _out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let ticks_per_frame = params.tpf;
    let fps = params.fps;
    let res = params.res;
    let scale = params.cell_size.scale();

    // Initialisation.
    let w = res[X];
    let h = res[Y];

    // Resources.
    let mut buffer: Vec<u32> = vec![0; w * h];
    let mut win = make_window(w, h, scale);
    let mut rng = rand::thread_rng();
    let mut world = World::new(params.res);

    // Limit to max ~60 fps update rate
    win.limit_update_rate(Some(std::time::Duration::from_micros(1000000 / fps)));

    // Main loop.
    section(term_width, "Running");
    while win.is_open() && !win.is_key_down(Key::Escape) {
        for _ in 0..ticks_per_frame {
            world.tick(&mut rng);
        }
        world.draw(&mut buffer);

        win.update_with_buffer(&buffer, w, h)
            .expect("Failed to render.")
    }
}

/// Initialise the input arguments.
fn initialisation(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    section(term_width, "Initialisation");
    sub_section(term_width, "args");
    args!(
        bin_path: PathBuf;
        input_dir: PathBuf;
        output_dir: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(input_dir.display(), "relative input path");
    report!(output_dir.display(), "relative output path");
    report!(params_path.display(), "parameters");

    sub_section(term_width, "directories");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join(input_dir)), Some(cwd.join(output_dir)))
        .expect("Failed to initialise directories.");
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (in_dir, out_dir, params_path)
}

/// Load the required files and form the input parameters.
fn load_parameters(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    section(term_width, "Parameters");
    sub_section(term_width, "Loading");
    Parameters::new_from_file(&in_dir.join(&params_path)).expect("Failed to load parameters file.")
}

// Create the main window.
fn make_window(width: usize, height: usize, scale: Scale) -> Window {
    debug_assert!(width > 0);
    debug_assert!(height > 0);

    let window_options = WindowOptions {
        borderless: true,
        title: true,
        resize: false,
        scale,
        scale_mode: ScaleMode::Center,
        topmost: true,
        transparency: true,
        none: false,
    };

    Window::new("Powder Game III", width, height, window_options).unwrap_or_else(|e| {
        panic!("Could not create gui window: {}", e);
    })
}
