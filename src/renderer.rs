// Copyright (c) 2026 Swayie
// SPDX-License-Identifier: MIT

use std::{
    io::{self, BufWriter, Write},
    path::Path,
};

use crate::{error::AppError, image::Image, magick, model::Color, ppm};

pub fn file(image: &Image, output: &Path) -> Result<(), AppError> {
    let extension = output
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| AppError::usage("output file must have an extension"))?;

    // PPM is supported natively; other formats are delegated to ImageMagick.
    if extension == "ppm" {
        ppm::write(image, output)
    } else {
        magick::convert(image, output)
    }
}

pub fn terminal(image: &Image) -> Result<(), AppError> {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for row in &image.pixels {
        for symbol in row {
            write_background(&mut writer, image.palette[symbol])?;
        }

        writer
            .write_all(b"\x1b[0m\n")
            .map_err(|error| AppError::io("cannot write terminal output", error))?;
    }

    writer
        .write_all(b"\x1b[0m")
        .map_err(|error| AppError::io("cannot write terminal output", error))?;
    writer
        .flush()
        .map_err(|error| AppError::io("cannot flush terminal output", error))
}

fn write_background<W: Write>(writer: &mut W, color: Color) -> Result<(), AppError> {
    write!(writer, "\x1b[48;2;{};{};{}m  ", color.r, color.g, color.b)
        .map_err(|error| AppError::io("cannot write terminal output", error))
}