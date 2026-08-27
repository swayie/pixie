mod app;
mod auto;
mod cli;
mod error;
mod image;
mod magick;
mod model;
mod parser;
mod ppm;
mod renderer;

use std::process;

fn main() {
    if let Err(error) = app::run() {
        error.report();
        process::exit(1);
    }
}
